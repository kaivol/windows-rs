#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2A_Impl: ::windows_core::BaseImpl + IDirectInputA_Impl {
    fn FindDevice(this: &Self::This, param0: *const ::windows_core::GUID, param1: &::windows_core::PCSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInput2A {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectInputA);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput2A_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInput2A {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: ::windows_core::PCSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindDevice(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        IDirectInput2A_Vtbl { base__: <IDirectInputA as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FindDevice: FindDevice::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2W_Impl: ::windows_core::BaseImpl + IDirectInputW_Impl {
    fn FindDevice(this: &Self::This, param0: *const ::windows_core::GUID, param1: &::windows_core::PCWSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInput2W {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectInputW);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput2W_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInput2W {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: ::windows_core::PCWSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindDevice(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        IDirectInput2W_Vtbl { base__: <IDirectInputW as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FindDevice: FindDevice::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7A_Impl: ::windows_core::BaseImpl + IDirectInput2A_Impl {
    fn CreateDeviceEx(this: &Self::This, param0: *const ::windows_core::GUID, param1: *const ::windows_core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInput7A {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectInput2A);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput7A_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInput7A {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDeviceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput7A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *const ::windows_core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeviceEx(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        IDirectInput7A_Vtbl { base__: <IDirectInput2A as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDeviceEx: CreateDeviceEx::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7W_Impl: ::windows_core::BaseImpl + IDirectInput2W_Impl {
    fn CreateDeviceEx(this: &Self::This, param0: *const ::windows_core::GUID, param1: *const ::windows_core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInput7W {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectInput2W);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput7W_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInput7W {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDeviceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput7W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *const ::windows_core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeviceEx(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        IDirectInput7W_Vtbl { base__: <IDirectInput2W as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDeviceEx: CreateDeviceEx::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput8A_Impl: ::windows_core::BaseImpl {
    fn CreateDevice(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8A>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumDevices(this: &Self::This, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>;
    fn GetDeviceStatus(this: &Self::This, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RunControlPanel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::Result<()>;
    fn FindDevice(this: &Self::This, param0: *const ::windows_core::GUID, param1: &::windows_core::PCSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumDevicesBySemantics(this: &Self::This, param0: &::windows_core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows_core::Result<()>;
    fn ConfigureDevices(this: &Self::This, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInput8A {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInput8A {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDevice(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDevices(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunControlPanel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn FindDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: ::windows_core::PCSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindDevice(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDevicesBySemantics(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn ConfigureDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureDevices(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        IDirectInput8A_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            FindDevice: FindDevice::<Identity, Impl, OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Identity, Impl, OFFSET>,
            ConfigureDevices: ConfigureDevices::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput8W_Impl: ::windows_core::BaseImpl {
    fn CreateDevice(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8W>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumDevices(this: &Self::This, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>;
    fn GetDeviceStatus(this: &Self::This, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RunControlPanel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::Result<()>;
    fn FindDevice(this: &Self::This, param0: *const ::windows_core::GUID, param1: &::windows_core::PCWSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumDevicesBySemantics(this: &Self::This, param0: &::windows_core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows_core::Result<()>;
    fn ConfigureDevices(this: &Self::This, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInput8W {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInput8W {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDevice(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDevices(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunControlPanel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn FindDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: ::windows_core::PCWSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindDevice(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDevicesBySemantics(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn ConfigureDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureDevices(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        IDirectInput8W_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            FindDevice: FindDevice::<Identity, Impl, OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Identity, Impl, OFFSET>,
            ConfigureDevices: ConfigureDevices::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputA_Impl: ::windows_core::BaseImpl {
    fn CreateDevice(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceA>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumDevices(this: &Self::This, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>;
    fn GetDeviceStatus(this: &Self::This, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RunControlPanel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputA {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputA {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDevice(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDevices(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunControlPanel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IDirectInputA_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice2A_Impl: ::windows_core::BaseImpl + IDirectInputDeviceA_Impl {
    fn CreateEffect(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumEffects(this: &Self::This, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn GetEffectInfo(this: &Self::This, param0: *mut DIEFFECTINFOA, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetForceFeedbackState(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn SendForceFeedbackCommand(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn EnumCreatedEffectObjects(this: &Self::This, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn Escape(this: &Self::This, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()>;
    fn Poll(this: &Self::This) -> ::windows_core::Result<()>;
    fn SendDeviceData(this: &Self::This, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputDevice2A {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectInputDeviceA);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputDevice2A {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateEffect(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumEffects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForceFeedbackState(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendForceFeedbackCommand(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumCreatedEffectObjects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Poll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Poll(this).into())
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendDeviceData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        IDirectInputDevice2A_Vtbl {
            base__: <IDirectInputDeviceA as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            EnumEffects: EnumEffects::<Identity, Impl, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            Poll: Poll::<Identity, Impl, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice2W_Impl: ::windows_core::BaseImpl + IDirectInputDeviceW_Impl {
    fn CreateEffect(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumEffects(this: &Self::This, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn GetEffectInfo(this: &Self::This, param0: *mut DIEFFECTINFOW, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetForceFeedbackState(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn SendForceFeedbackCommand(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn EnumCreatedEffectObjects(this: &Self::This, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn Escape(this: &Self::This, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()>;
    fn Poll(this: &Self::This) -> ::windows_core::Result<()>;
    fn SendDeviceData(this: &Self::This, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputDevice2W {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectInputDeviceW);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputDevice2W {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateEffect(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumEffects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForceFeedbackState(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendForceFeedbackCommand(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumCreatedEffectObjects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Poll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Poll(this).into())
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendDeviceData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        IDirectInputDevice2W_Vtbl {
            base__: <IDirectInputDeviceW as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            EnumEffects: EnumEffects::<Identity, Impl, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            Poll: Poll::<Identity, Impl, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7A_Impl: ::windows_core::BaseImpl + IDirectInputDevice2A_Impl {
    fn EnumEffectsInFile(this: &Self::This, param0: &::windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>;
    fn WriteEffectToFile(this: &Self::This, param0: &::windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputDevice7A {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectInputDevice2A);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice7A_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputDevice7A {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice7A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumEffectsInFile(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice7A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEffectToFile(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        IDirectInputDevice7A_Vtbl {
            base__: <IDirectInputDevice2A as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7W_Impl: ::windows_core::BaseImpl + IDirectInputDevice2W_Impl {
    fn EnumEffectsInFile(this: &Self::This, param0: &::windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>;
    fn WriteEffectToFile(this: &Self::This, param0: &::windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputDevice7W {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectInputDevice2W);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice7W_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputDevice7W {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice7W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumEffectsInFile(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice7W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEffectToFile(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        IDirectInputDevice7W_Vtbl {
            base__: <IDirectInputDevice2W as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice8A_Impl: ::windows_core::BaseImpl {
    fn GetCapabilities(this: &Self::This, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()>;
    fn EnumObjects(this: &Self::This, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()>;
    fn SetProperty(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()>;
    fn Acquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unacquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDeviceState(this: &Self::This, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDeviceData(this: &Self::This, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()>;
    fn SetDataFormat(this: &Self::This, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()>;
    fn SetEventNotification(this: &Self::This, param0: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn GetObjectInfo(this: &Self::This, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn GetDeviceInfo(this: &Self::This, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::Result<()>;
    fn RunControlPanel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn CreateEffect(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumEffects(this: &Self::This, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn GetEffectInfo(this: &Self::This, param0: *mut DIEFFECTINFOA, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetForceFeedbackState(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn SendForceFeedbackCommand(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn EnumCreatedEffectObjects(this: &Self::This, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn Escape(this: &Self::This, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()>;
    fn Poll(this: &Self::This) -> ::windows_core::Result<()>;
    fn SendDeviceData(this: &Self::This, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()>;
    fn EnumEffectsInFile(this: &Self::This, param0: &::windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>;
    fn WriteEffectToFile(this: &Self::This, param0: &::windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::Result<()>;
    fn BuildActionMap(this: &Self::This, param0: *mut DIACTIONFORMATA, param1: &::windows_core::PCSTR, param2: u32) -> ::windows_core::Result<()>;
    fn SetActionMap(this: &Self::This, param0: *mut DIACTIONFORMATA, param1: &::windows_core::PCSTR, param2: u32) -> ::windows_core::Result<()>;
    fn GetImageInfo(this: &Self::This, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputDevice8A {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputDevice8A {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCapabilities(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumObjects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Acquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Acquire(this).into())
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unacquire(this).into())
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceState(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDataFormat(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventNotification(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceInfo(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunControlPanel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn CreateEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateEffect(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumEffects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForceFeedbackState(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendForceFeedbackCommand(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumCreatedEffectObjects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Poll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Poll(this).into())
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendDeviceData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumEffectsInFile(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEffectToFile(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn BuildActionMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: ::windows_core::PCSTR, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BuildActionMap(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn SetActionMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: ::windows_core::PCSTR, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActionMap(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetImageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImageInfo(this, ::core::mem::transmute_copy(&param0)).into())
        }
        IDirectInputDevice8A_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, Impl, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, Impl, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            EnumEffects: EnumEffects::<Identity, Impl, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            Poll: Poll::<Identity, Impl, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, Impl, OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
            BuildActionMap: BuildActionMap::<Identity, Impl, OFFSET>,
            SetActionMap: SetActionMap::<Identity, Impl, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice8W_Impl: ::windows_core::BaseImpl {
    fn GetCapabilities(this: &Self::This, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()>;
    fn EnumObjects(this: &Self::This, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()>;
    fn SetProperty(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()>;
    fn Acquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unacquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDeviceState(this: &Self::This, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDeviceData(this: &Self::This, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()>;
    fn SetDataFormat(this: &Self::This, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()>;
    fn SetEventNotification(this: &Self::This, param0: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn GetObjectInfo(this: &Self::This, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn GetDeviceInfo(this: &Self::This, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::Result<()>;
    fn RunControlPanel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn CreateEffect(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumEffects(this: &Self::This, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn GetEffectInfo(this: &Self::This, param0: *mut DIEFFECTINFOW, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetForceFeedbackState(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn SendForceFeedbackCommand(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn EnumCreatedEffectObjects(this: &Self::This, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn Escape(this: &Self::This, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()>;
    fn Poll(this: &Self::This) -> ::windows_core::Result<()>;
    fn SendDeviceData(this: &Self::This, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()>;
    fn EnumEffectsInFile(this: &Self::This, param0: &::windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>;
    fn WriteEffectToFile(this: &Self::This, param0: &::windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::Result<()>;
    fn BuildActionMap(this: &Self::This, param0: *mut DIACTIONFORMATW, param1: &::windows_core::PCWSTR, param2: u32) -> ::windows_core::Result<()>;
    fn SetActionMap(this: &Self::This, param0: *mut DIACTIONFORMATW, param1: &::windows_core::PCWSTR, param2: u32) -> ::windows_core::Result<()>;
    fn GetImageInfo(this: &Self::This, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputDevice8W {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputDevice8W {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCapabilities(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumObjects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Acquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Acquire(this).into())
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unacquire(this).into())
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceState(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDataFormat(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventNotification(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceInfo(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunControlPanel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn CreateEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateEffect(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumEffects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForceFeedbackState(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendForceFeedbackCommand(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumCreatedEffectObjects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Poll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Poll(this).into())
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendDeviceData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumEffectsInFile(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEffectToFile(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn BuildActionMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: ::windows_core::PCWSTR, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BuildActionMap(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn SetActionMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: ::windows_core::PCWSTR, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActionMap(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetImageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImageInfo(this, ::core::mem::transmute_copy(&param0)).into())
        }
        IDirectInputDevice8W_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, Impl, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, Impl, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            EnumEffects: EnumEffects::<Identity, Impl, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            Poll: Poll::<Identity, Impl, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, Impl, OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
            BuildActionMap: BuildActionMap::<Identity, Impl, OFFSET>,
            SetActionMap: SetActionMap::<Identity, Impl, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDeviceA_Impl: ::windows_core::BaseImpl {
    fn GetCapabilities(this: &Self::This, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()>;
    fn EnumObjects(this: &Self::This, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()>;
    fn SetProperty(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()>;
    fn Acquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unacquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDeviceState(this: &Self::This, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDeviceData(this: &Self::This, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()>;
    fn SetDataFormat(this: &Self::This, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()>;
    fn SetEventNotification(this: &Self::This, param0: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn GetObjectInfo(this: &Self::This, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn GetDeviceInfo(this: &Self::This, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::Result<()>;
    fn RunControlPanel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputDeviceA {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputDeviceA {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCapabilities(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumObjects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Acquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Acquire(this).into())
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unacquire(this).into())
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceState(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDataFormat(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventNotification(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceInfo(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunControlPanel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        IDirectInputDeviceA_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, Impl, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, Impl, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDeviceW_Impl: ::windows_core::BaseImpl {
    fn GetCapabilities(this: &Self::This, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()>;
    fn EnumObjects(this: &Self::This, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()>;
    fn SetProperty(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()>;
    fn Acquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unacquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDeviceState(this: &Self::This, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDeviceData(this: &Self::This, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()>;
    fn SetDataFormat(this: &Self::This, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()>;
    fn SetEventNotification(this: &Self::This, param0: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn GetObjectInfo(this: &Self::This, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn GetDeviceInfo(this: &Self::This, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::Result<()>;
    fn RunControlPanel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputDeviceW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputDeviceW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCapabilities(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumObjects(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Acquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Acquire(this).into())
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unacquire(this).into())
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceState(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDataFormat(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventNotification(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceInfo(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunControlPanel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        IDirectInputDeviceW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, Impl, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, Impl, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputEffect_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetEffectGuid(this: &Self::This, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetParameters(this: &Self::This, param0: *mut DIEFFECT, param1: u32) -> ::windows_core::Result<()>;
    fn SetParameters(this: &Self::This, param0: *mut DIEFFECT, param1: u32) -> ::windows_core::Result<()>;
    fn Start(this: &Self::This, param0: u32, param1: u32) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetEffectStatus(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn Download(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unload(this: &Self::This) -> ::windows_core::Result<()>;
    fn Escape(this: &Self::This, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetEffectGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectGuid(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParameters(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameters(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn GetEffectStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Download<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Download(this).into())
        }
        unsafe extern "system" fn Unload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unload(this).into())
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&param0)).into())
        }
        IDirectInputEffect_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetEffectGuid: GetEffectGuid::<Identity, Impl, OFFSET>,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            GetEffectStatus: GetEffectStatus::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            Unload: Unload::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectInputEffectDriver_Impl: ::windows_core::BaseImpl {
    fn DeviceID(this: &Self::This, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetVersions(this: &Self::This, param0: *mut DIDRIVERVERSIONS) -> ::windows_core::Result<()>;
    fn Escape(this: &Self::This, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows_core::Result<()>;
    fn SetGain(this: &Self::This, param0: u32, param1: u32) -> ::windows_core::Result<()>;
    fn SendForceFeedbackCommand(this: &Self::This, param0: u32, param1: u32) -> ::windows_core::Result<()>;
    fn GetForceFeedbackState(this: &Self::This, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows_core::Result<()>;
    fn DownloadEffect(this: &Self::This, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows_core::Result<()>;
    fn DestroyEffect(this: &Self::This, param0: u32, param1: u32) -> ::windows_core::Result<()>;
    fn StartEffect(this: &Self::This, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows_core::Result<()>;
    fn StopEffect(this: &Self::This, param0: u32, param1: u32) -> ::windows_core::Result<()>;
    fn GetEffectStatus(this: &Self::This, param0: u32, param1: u32, param2: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectInputEffectDriver {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputEffectDriver {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceID(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn GetVersions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVersions(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn SetGain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGain(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendForceFeedbackCommand(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForceFeedbackState(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn DownloadEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DownloadEffect(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn DestroyEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyEffect(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn StartEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartEffect(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn StopEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopEffect(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetEffectStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectStatus(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        IDirectInputEffectDriver_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceID: DeviceID::<Identity, Impl, OFFSET>,
            GetVersions: GetVersions::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            SetGain: SetGain::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            DownloadEffect: DownloadEffect::<Identity, Impl, OFFSET>,
            DestroyEffect: DestroyEffect::<Identity, Impl, OFFSET>,
            StartEffect: StartEffect::<Identity, Impl, OFFSET>,
            StopEffect: StopEffect::<Identity, Impl, OFFSET>,
            GetEffectStatus: GetEffectStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IDirectInputJoyConfig_Impl: ::windows_core::BaseImpl {
    fn Acquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unacquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SendNotify(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumTypes(this: &Self::This, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetTypeInfo(this: &Self::This, param0: &::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::Result<()>;
    fn SetTypeInfo(this: &Self::This, param0: &::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::Result<()>;
    fn DeleteType(this: &Self::This, param0: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetConfig(this: &Self::This, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::Result<()>;
    fn SetConfig(this: &Self::This, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::Result<()>;
    fn DeleteConfig(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetUserValues(this: &Self::This, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::Result<()>;
    fn SetUserValues(this: &Self::This, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::Result<()>;
    fn AddNewHardware(this: &Self::This, param0: super::super::Foundation::HWND, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OpenTypeKey(this: &Self::This, param0: &::windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()>;
    fn OpenConfigKey(this: &Self::This, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::Iids for IDirectInputJoyConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputJoyConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Acquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Acquire(this).into())
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unacquire(this).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SendNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendNotify(this).into())
        }
        unsafe extern "system" fn EnumTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumTypes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTypeInfo(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn SetTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTypeInfo(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn DeleteType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteType(this, ::core::mem::transmute(&param0)).into())
        }
        unsafe extern "system" fn GetConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConfig(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn SetConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConfig(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn DeleteConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteConfig(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetUserValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUserValues(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetUserValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserValues(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn AddNewHardware<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNewHardware(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn OpenTypeKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenTypeKey(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn OpenConfigKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenConfigKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        IDirectInputJoyConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SendNotify: SendNotify::<Identity, Impl, OFFSET>,
            EnumTypes: EnumTypes::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            SetTypeInfo: SetTypeInfo::<Identity, Impl, OFFSET>,
            DeleteType: DeleteType::<Identity, Impl, OFFSET>,
            GetConfig: GetConfig::<Identity, Impl, OFFSET>,
            SetConfig: SetConfig::<Identity, Impl, OFFSET>,
            DeleteConfig: DeleteConfig::<Identity, Impl, OFFSET>,
            GetUserValues: GetUserValues::<Identity, Impl, OFFSET>,
            SetUserValues: SetUserValues::<Identity, Impl, OFFSET>,
            AddNewHardware: AddNewHardware::<Identity, Impl, OFFSET>,
            OpenTypeKey: OpenTypeKey::<Identity, Impl, OFFSET>,
            OpenConfigKey: OpenConfigKey::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IDirectInputJoyConfig8_Impl: ::windows_core::BaseImpl {
    fn Acquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unacquire(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SendNotify(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumTypes(this: &Self::This, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetTypeInfo(this: &Self::This, param0: &::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::Result<()>;
    fn SetTypeInfo(this: &Self::This, param0: &::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DeleteType(this: &Self::This, param0: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetConfig(this: &Self::This, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::Result<()>;
    fn SetConfig(this: &Self::This, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::Result<()>;
    fn DeleteConfig(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetUserValues(this: &Self::This, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::Result<()>;
    fn SetUserValues(this: &Self::This, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::Result<()>;
    fn AddNewHardware(this: &Self::This, param0: super::super::Foundation::HWND, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OpenTypeKey(this: &Self::This, param0: &::windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()>;
    fn OpenAppStatusKey(this: &Self::This, param0: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::Iids for IDirectInputJoyConfig8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputJoyConfig8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Acquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Acquire(this).into())
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unacquire(this).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SendNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendNotify(this).into())
        }
        unsafe extern "system" fn EnumTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumTypes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTypeInfo(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn SetTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTypeInfo(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into())
        }
        unsafe extern "system" fn DeleteType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteType(this, ::core::mem::transmute(&param0)).into())
        }
        unsafe extern "system" fn GetConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConfig(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn SetConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConfig(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn DeleteConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteConfig(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetUserValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUserValues(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetUserValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserValues(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn AddNewHardware<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNewHardware(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn OpenTypeKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenTypeKey(this, ::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn OpenAppStatusKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenAppStatusKey(this, ::core::mem::transmute_copy(&param0)).into())
        }
        IDirectInputJoyConfig8_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SendNotify: SendNotify::<Identity, Impl, OFFSET>,
            EnumTypes: EnumTypes::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            SetTypeInfo: SetTypeInfo::<Identity, Impl, OFFSET>,
            DeleteType: DeleteType::<Identity, Impl, OFFSET>,
            GetConfig: GetConfig::<Identity, Impl, OFFSET>,
            SetConfig: SetConfig::<Identity, Impl, OFFSET>,
            DeleteConfig: DeleteConfig::<Identity, Impl, OFFSET>,
            GetUserValues: GetUserValues::<Identity, Impl, OFFSET>,
            SetUserValues: SetUserValues::<Identity, Impl, OFFSET>,
            AddNewHardware: AddNewHardware::<Identity, Impl, OFFSET>,
            OpenTypeKey: OpenTypeKey::<Identity, Impl, OFFSET>,
            OpenAppStatusKey: OpenAppStatusKey::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputW_Impl: ::windows_core::BaseImpl {
    fn CreateDevice(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceW>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumDevices(this: &Self::This, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>;
    fn GetDeviceStatus(this: &Self::This, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RunControlPanel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectInputW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectInputW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDevice(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDevices(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunControlPanel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IDirectInputW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    };
}
