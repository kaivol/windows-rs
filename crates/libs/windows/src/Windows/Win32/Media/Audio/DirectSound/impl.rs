#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSound_Impl: ::windows_core::BaseImpl {
    fn CreateSoundBuffer(this: &Self::This, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::core::option::Option<IDirectSoundBuffer>, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, pdscaps: *mut DSCAPS) -> ::windows_core::Result<()>;
    fn DuplicateSoundBuffer(this: &Self::This, pdsbufferoriginal: ::core::option::Option<&IDirectSoundBuffer>) -> ::windows_core::Result<IDirectSoundBuffer>;
    fn SetCooperativeLevel(this: &Self::This, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows_core::Result<()>;
    fn Compact(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetSpeakerConfig(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetSpeakerConfig(this: &Self::This, dwspeakerconfig: u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, pcguiddevice: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectSound {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSound {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSoundBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSoundBuffer(this, ::core::mem::transmute_copy(&pcdsbufferdesc), ::core::mem::transmute_copy(&ppdsbuffer), ::windows_core::from_raw_borrowed(&punkouter)).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscaps: *mut DSCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&pdscaps)).into())
        }
        unsafe extern "system" fn DuplicateSoundBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsbufferoriginal: *mut ::core::ffi::c_void, ppdsbufferduplicate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DuplicateSoundBuffer(this, ::windows_core::from_raw_borrowed(&pdsbufferoriginal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdsbufferduplicate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwlevel)).into())
        }
        unsafe extern "system" fn Compact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compact(this).into())
        }
        unsafe extern "system" fn GetSpeakerConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwspeakerconfig: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSpeakerConfig(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwspeakerconfig, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSpeakerConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwspeakerconfig: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSpeakerConfig(this, ::core::mem::transmute_copy(&dwspeakerconfig)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&pcguiddevice)).into())
        }
        IDirectSound_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSoundBuffer: CreateSoundBuffer::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            DuplicateSoundBuffer: DuplicateSoundBuffer::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            Compact: Compact::<Identity, Impl, OFFSET>,
            GetSpeakerConfig: GetSpeakerConfig::<Identity, Impl, OFFSET>,
            SetSpeakerConfig: SetSpeakerConfig::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DBuffer_Impl: ::windows_core::BaseImpl {
    fn GetAllParameters(this: &Self::This, pds3dbuffer: *mut DS3DBUFFER) -> ::windows_core::Result<()>;
    fn GetConeAngles(this: &Self::This, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows_core::Result<()>;
    fn GetConeOrientation(this: &Self::This) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetConeOutsideVolume(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetMaxDistance(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetMinDistance(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetMode(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPosition(this: &Self::This) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetVelocity(this: &Self::This) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn SetAllParameters(this: &Self::This, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetConeAngles(this: &Self::This, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetConeOrientation(this: &Self::This, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetConeOutsideVolume(this: &Self::This, lconeoutsidevolume: i32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetMaxDistance(this: &Self::This, flmaxdistance: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetMinDistance(this: &Self::This, flmindistance: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetMode(this: &Self::This, dwmode: u32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetPosition(this: &Self::This, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetVelocity(this: &Self::This, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::Iids for IDirectSound3DBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSound3DBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pds3dbuffer: *mut DS3DBUFFER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllParameters(this, ::core::mem::transmute_copy(&pds3dbuffer)).into())
        }
        unsafe extern "system" fn GetConeAngles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConeAngles(this, ::core::mem::transmute_copy(&pdwinsideconeangle), ::core::mem::transmute_copy(&pdwoutsideconeangle)).into())
        }
        unsafe extern "system" fn GetConeOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConeOrientation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvorientation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConeOutsideVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plconeoutsidevolume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConeOutsideVolume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plconeoutsidevolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxDistance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflmaxdistance: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxDistance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflmaxdistance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMinDistance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflmindistance: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinDistance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflmindistance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVelocity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvvelocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcds3dbuffer), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetConeAngles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConeAngles(this, ::core::mem::transmute_copy(&dwinsideconeangle), ::core::mem::transmute_copy(&dwoutsideconeangle), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetConeOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConeOrientation(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetConeOutsideVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lconeoutsidevolume: i32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConeOutsideVolume(this, ::core::mem::transmute_copy(&lconeoutsidevolume), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetMaxDistance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flmaxdistance: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxDistance(this, ::core::mem::transmute_copy(&flmaxdistance), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetMinDistance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flmindistance: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinDistance(this, ::core::mem::transmute_copy(&flmindistance), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMode(this, ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPosition(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVelocity(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into())
        }
        IDirectSound3DBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            GetConeAngles: GetConeAngles::<Identity, Impl, OFFSET>,
            GetConeOrientation: GetConeOrientation::<Identity, Impl, OFFSET>,
            GetConeOutsideVolume: GetConeOutsideVolume::<Identity, Impl, OFFSET>,
            GetMaxDistance: GetMaxDistance::<Identity, Impl, OFFSET>,
            GetMinDistance: GetMinDistance::<Identity, Impl, OFFSET>,
            GetMode: GetMode::<Identity, Impl, OFFSET>,
            GetPosition: GetPosition::<Identity, Impl, OFFSET>,
            GetVelocity: GetVelocity::<Identity, Impl, OFFSET>,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            SetConeAngles: SetConeAngles::<Identity, Impl, OFFSET>,
            SetConeOrientation: SetConeOrientation::<Identity, Impl, OFFSET>,
            SetConeOutsideVolume: SetConeOutsideVolume::<Identity, Impl, OFFSET>,
            SetMaxDistance: SetMaxDistance::<Identity, Impl, OFFSET>,
            SetMinDistance: SetMinDistance::<Identity, Impl, OFFSET>,
            SetMode: SetMode::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            SetVelocity: SetVelocity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DListener_Impl: ::windows_core::BaseImpl {
    fn GetAllParameters(this: &Self::This, plistener: *mut DS3DLISTENER) -> ::windows_core::Result<()>;
    fn GetDistanceFactor(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetDopplerFactor(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetOrientation(this: &Self::This, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::Result<()>;
    fn GetPosition(this: &Self::This) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetRolloffFactor(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetVelocity(this: &Self::This) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn SetAllParameters(this: &Self::This, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetDistanceFactor(this: &Self::This, fldistancefactor: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetDopplerFactor(this: &Self::This, fldopplerfactor: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetOrientation(this: &Self::This, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetPosition(this: &Self::This, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetRolloffFactor(this: &Self::This, flrollofffactor: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetVelocity(this: &Self::This, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn CommitDeferredSettings(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::Iids for IDirectSound3DListener {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSound3DListener {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plistener: *mut DS3DLISTENER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllParameters(this, ::core::mem::transmute_copy(&plistener)).into())
        }
        unsafe extern "system" fn GetDistanceFactor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfldistancefactor: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDistanceFactor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfldistancefactor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDopplerFactor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfldopplerfactor: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDopplerFactor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfldopplerfactor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOrientation(this, ::core::mem::transmute_copy(&pvorientfront), ::core::mem::transmute_copy(&pvorienttop)).into())
        }
        unsafe extern "system" fn GetPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRolloffFactor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflrollofffactor: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRolloffFactor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflrollofffactor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVelocity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvvelocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pclistener), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetDistanceFactor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fldistancefactor: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDistanceFactor(this, ::core::mem::transmute_copy(&fldistancefactor), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetDopplerFactor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fldopplerfactor: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDopplerFactor(this, ::core::mem::transmute_copy(&fldopplerfactor), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOrientation(this, ::core::mem::transmute_copy(&xfront), ::core::mem::transmute_copy(&yfront), ::core::mem::transmute_copy(&zfront), ::core::mem::transmute_copy(&xtop), ::core::mem::transmute_copy(&ytop), ::core::mem::transmute_copy(&ztop), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPosition(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetRolloffFactor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flrollofffactor: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRolloffFactor(this, ::core::mem::transmute_copy(&flrollofffactor), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn SetVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVelocity(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into())
        }
        unsafe extern "system" fn CommitDeferredSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitDeferredSettings(this).into())
        }
        IDirectSound3DListener_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            GetDistanceFactor: GetDistanceFactor::<Identity, Impl, OFFSET>,
            GetDopplerFactor: GetDopplerFactor::<Identity, Impl, OFFSET>,
            GetOrientation: GetOrientation::<Identity, Impl, OFFSET>,
            GetPosition: GetPosition::<Identity, Impl, OFFSET>,
            GetRolloffFactor: GetRolloffFactor::<Identity, Impl, OFFSET>,
            GetVelocity: GetVelocity::<Identity, Impl, OFFSET>,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            SetDistanceFactor: SetDistanceFactor::<Identity, Impl, OFFSET>,
            SetDopplerFactor: SetDopplerFactor::<Identity, Impl, OFFSET>,
            SetOrientation: SetOrientation::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            SetRolloffFactor: SetRolloffFactor::<Identity, Impl, OFFSET>,
            SetVelocity: SetVelocity::<Identity, Impl, OFFSET>,
            CommitDeferredSettings: CommitDeferredSettings::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSound8_Impl: ::windows_core::BaseImpl + IDirectSound_Impl {
    fn VerifyCertification(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectSound8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectSound);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSound8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn VerifyCertification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSound8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcertified: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VerifyCertification(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcertified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDirectSound8_Vtbl { base__: <IDirectSound as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, VerifyCertification: VerifyCertification::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundBuffer_Impl: ::windows_core::BaseImpl {
    fn GetCaps(this: &Self::This, pdsbuffercaps: *mut DSBCAPS) -> ::windows_core::Result<()>;
    fn GetCurrentPosition(this: &Self::This, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows_core::Result<()>;
    fn GetFormat(this: &Self::This, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_core::Result<()>;
    fn GetVolume(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetPan(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetFrequency(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Initialize(this: &Self::This, pdirectsound: ::core::option::Option<&IDirectSound>, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows_core::Result<()>;
    fn Lock(this: &Self::This, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn Play(this: &Self::This, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn SetCurrentPosition(this: &Self::This, dwnewposition: u32) -> ::windows_core::Result<()>;
    fn SetFormat(this: &Self::This, pcfxformat: *const super::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn SetVolume(this: &Self::This, lvolume: i32) -> ::windows_core::Result<()>;
    fn SetPan(this: &Self::This, lpan: i32) -> ::windows_core::Result<()>;
    fn SetFrequency(this: &Self::This, dwfrequency: u32) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsbuffercaps: *mut DSBCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&pdsbuffercaps)).into())
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentPosition(this, ::core::mem::transmute_copy(&pdwcurrentplaycursor), ::core::mem::transmute_copy(&pdwcurrentwritecursor)).into())
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFormat(this, ::core::mem::transmute_copy(&pwfxformat), ::core::mem::transmute_copy(&dwsizeallocated), ::core::mem::transmute_copy(&pdwsizewritten)).into())
        }
        unsafe extern "system" fn GetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVolume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpan: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPan(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpan, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfrequency: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrequency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwfrequency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectsound: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pdirectsound), ::core::mem::transmute_copy(&pcdsbufferdesc)).into())
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&dwoffset), ::core::mem::transmute_copy(&dwbytes), ::core::mem::transmute_copy(&ppvaudioptr1), ::core::mem::transmute_copy(&pdwaudiobytes1), ::core::mem::transmute_copy(&ppvaudioptr2), ::core::mem::transmute_copy(&pdwaudiobytes2), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn Play<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Play(this, ::core::mem::transmute_copy(&dwreserved1), ::core::mem::transmute_copy(&dwpriority), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnewposition: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCurrentPosition(this, ::core::mem::transmute_copy(&dwnewposition)).into())
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcfxformat: *const super::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormat(this, ::core::mem::transmute_copy(&pcfxformat)).into())
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolume(this, ::core::mem::transmute_copy(&lvolume)).into())
        }
        unsafe extern "system" fn SetPan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpan: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPan(this, ::core::mem::transmute_copy(&lpan)).into())
        }
        unsafe extern "system" fn SetFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfrequency: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrequency(this, ::core::mem::transmute_copy(&dwfrequency)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this, ::core::mem::transmute_copy(&pvaudioptr1), ::core::mem::transmute_copy(&dwaudiobytes1), ::core::mem::transmute_copy(&pvaudioptr2), ::core::mem::transmute_copy(&dwaudiobytes2)).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this).into())
        }
        IDirectSoundBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetVolume: GetVolume::<Identity, Impl, OFFSET>,
            GetPan: GetPan::<Identity, Impl, OFFSET>,
            GetFrequency: GetFrequency::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            SetPan: SetPan::<Identity, Impl, OFFSET>,
            SetFrequency: SetFrequency::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundBuffer8_Impl: ::windows_core::BaseImpl + IDirectSoundBuffer_Impl {
    fn SetFX(this: &Self::This, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows_core::Result<()>;
    fn AcquireResources(this: &Self::This, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows_core::Result<()>;
    fn GetObjectInPath(this: &Self::This, rguidobject: *const ::windows_core::GUID, dwindex: u32, rguidinterface: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundBuffer8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectSoundBuffer);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundBuffer8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFX(this, ::core::mem::transmute_copy(&dweffectscount), ::core::mem::transmute_copy(&pdsfxdesc), ::core::mem::transmute_copy(&pdwresultcodes)).into())
        }
        unsafe extern "system" fn AcquireResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireResources(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dweffectscount), ::core::mem::transmute_copy(&pdwresultcodes)).into())
        }
        unsafe extern "system" fn GetObjectInPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundBuffer8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows_core::GUID, dwindex: u32, rguidinterface: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectInPath(this, ::core::mem::transmute_copy(&rguidobject), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&rguidinterface), ::core::mem::transmute_copy(&ppobject)).into())
        }
        IDirectSoundBuffer8_Vtbl {
            base__: <IDirectSoundBuffer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFX: SetFX::<Identity, Impl, OFFSET>,
            AcquireResources: AcquireResources::<Identity, Impl, OFFSET>,
            GetObjectInPath: GetObjectInPath::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundCapture_Impl: ::windows_core::BaseImpl {
    fn CreateCaptureBuffer(this: &Self::This, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::core::option::Option<IDirectSoundCaptureBuffer>, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This) -> ::windows_core::Result<DSCCAPS>;
    fn Initialize(this: &Self::This, pcguiddevice: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundCapture {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCapture_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundCapture {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateCaptureBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCapture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCaptureBuffer(this, ::core::mem::transmute_copy(&pcdscbufferdesc), ::core::mem::transmute_copy(&ppdscbuffer), ::windows_core::from_raw_borrowed(&punkouter)).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCapture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsccaps: *mut DSCCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCaps(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdsccaps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCapture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&pcguiddevice)).into())
        }
        IDirectSoundCapture_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateCaptureBuffer: CreateCaptureBuffer::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundCaptureBuffer_Impl: ::windows_core::BaseImpl {
    fn GetCaps(this: &Self::This) -> ::windows_core::Result<DSCBCAPS>;
    fn GetCurrentPosition(this: &Self::This, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows_core::Result<()>;
    fn GetFormat(this: &Self::This, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Initialize(this: &Self::This, pdirectsoundcapture: ::core::option::Option<&IDirectSoundCapture>, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows_core::Result<()>;
    fn Lock(this: &Self::This, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn Start(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundCaptureBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundCaptureBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscbcaps: *mut DSCBCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCaps(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdscbcaps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentPosition(this, ::core::mem::transmute_copy(&pdwcaptureposition), ::core::mem::transmute_copy(&pdwreadposition)).into())
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFormat(this, ::core::mem::transmute_copy(&pwfxformat), ::core::mem::transmute_copy(&dwsizeallocated), ::core::mem::transmute_copy(&pdwsizewritten)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectsoundcapture: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pdirectsoundcapture), ::core::mem::transmute_copy(&pcdscbufferdesc)).into())
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&dwoffset), ::core::mem::transmute_copy(&dwbytes), ::core::mem::transmute_copy(&ppvaudioptr1), ::core::mem::transmute_copy(&pdwaudiobytes1), ::core::mem::transmute_copy(&ppvaudioptr2), ::core::mem::transmute_copy(&pdwaudiobytes2), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this, ::core::mem::transmute_copy(&pvaudioptr1), ::core::mem::transmute_copy(&dwaudiobytes1), ::core::mem::transmute_copy(&pvaudioptr2), ::core::mem::transmute_copy(&dwaudiobytes2)).into())
        }
        IDirectSoundCaptureBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundCaptureBuffer8_Impl: ::windows_core::BaseImpl + IDirectSoundCaptureBuffer_Impl {
    fn GetObjectInPath(this: &Self::This, rguidobject: *const ::windows_core::GUID, dwindex: u32, rguidinterface: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetFXStatus(this: &Self::This, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundCaptureBuffer8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectSoundCaptureBuffer);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundCaptureBuffer8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectInPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows_core::GUID, dwindex: u32, rguidinterface: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectInPath(this, ::core::mem::transmute_copy(&rguidobject), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&rguidinterface), ::core::mem::transmute_copy(&ppobject)).into())
        }
        unsafe extern "system" fn GetFXStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureBuffer8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFXStatus(this, ::core::mem::transmute_copy(&dweffectscount), ::core::mem::transmute_copy(&pdwfxstatus)).into())
        }
        IDirectSoundCaptureBuffer8_Vtbl {
            base__: <IDirectSoundCaptureBuffer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectInPath: GetObjectInPath::<Identity, Impl, OFFSET>,
            GetFXStatus: GetFXStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundCaptureFXAec_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pdscfxaec: *const DSCFXAec) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This) -> ::windows_core::Result<DSCFXAec>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectSoundCaptureFXAec {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundCaptureFXAec {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscfxaec: *const DSCFXAec) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pdscfxaec)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscfxaec: *mut DSCFXAec) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdscfxaec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IDirectSoundCaptureFXAec_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundCaptureFXNoiseSuppress_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This) -> ::windows_core::Result<DSCFXNoiseSuppress>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectSoundCaptureFXNoiseSuppress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundCaptureFXNoiseSuppress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdscfxnoisesuppress)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdscfxnoisesuppress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IDirectSoundCaptureFXNoiseSuppress_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundFXChorus_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdsfxchorus: *const DSFXChorus) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This, pdsfxchorus: *mut DSFXChorus) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundFXChorus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXChorus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFXChorus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXChorus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxchorus: *const DSFXChorus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdsfxchorus)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXChorus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxchorus: *mut DSFXChorus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllParameters(this, ::core::mem::transmute_copy(&pdsfxchorus)).into())
        }
        IDirectSoundFXChorus_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundFXCompressor_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdsfxcompressor: *const DSFXCompressor) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This, pdsfxcompressor: *mut DSFXCompressor) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundFXCompressor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXCompressor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFXCompressor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXCompressor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxcompressor: *const DSFXCompressor) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdsfxcompressor)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXCompressor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxcompressor: *mut DSFXCompressor) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllParameters(this, ::core::mem::transmute_copy(&pdsfxcompressor)).into())
        }
        IDirectSoundFXCompressor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundFXDistortion_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdsfxdistortion: *const DSFXDistortion) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This, pdsfxdistortion: *mut DSFXDistortion) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundFXDistortion {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXDistortion_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFXDistortion {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXDistortion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxdistortion: *const DSFXDistortion) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdsfxdistortion)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXDistortion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxdistortion: *mut DSFXDistortion) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllParameters(this, ::core::mem::transmute_copy(&pdsfxdistortion)).into())
        }
        IDirectSoundFXDistortion_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundFXEcho_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdsfxecho: *const DSFXEcho) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This, pdsfxecho: *mut DSFXEcho) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundFXEcho {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXEcho_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFXEcho {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXEcho_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxecho: *const DSFXEcho) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdsfxecho)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXEcho_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxecho: *mut DSFXEcho) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllParameters(this, ::core::mem::transmute_copy(&pdsfxecho)).into())
        }
        IDirectSoundFXEcho_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundFXFlanger_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdsfxflanger: *const DSFXFlanger) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This, pdsfxflanger: *mut DSFXFlanger) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectSoundFXFlanger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXFlanger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFXFlanger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXFlanger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxflanger: *const DSFXFlanger) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdsfxflanger)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXFlanger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxflanger: *mut DSFXFlanger) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllParameters(this, ::core::mem::transmute_copy(&pdsfxflanger)).into())
        }
        IDirectSoundFXFlanger_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundFXGargle_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdsfxgargle: *const DSFXGargle) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This) -> ::windows_core::Result<DSFXGargle>;
}
impl ::windows_core::Iids for IDirectSoundFXGargle {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXGargle_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFXGargle {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXGargle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxgargle: *const DSFXGargle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdsfxgargle)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXGargle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxgargle: *mut DSFXGargle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdsfxgargle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDirectSoundFXGargle_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundFXI3DL2Reverb_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows_core::Result<()>;
    fn SetPreset(this: &Self::This, dwpreset: u32) -> ::windows_core::Result<()>;
    fn GetPreset(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetQuality(this: &Self::This, lquality: i32) -> ::windows_core::Result<()>;
    fn GetQuality(this: &Self::This) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IDirectSoundFXI3DL2Reverb {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFXI3DL2Reverb {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdsfxi3dl2reverb)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllParameters(this, ::core::mem::transmute_copy(&pdsfxi3dl2reverb)).into())
        }
        unsafe extern "system" fn SetPreset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpreset: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreset(this, ::core::mem::transmute_copy(&dwpreset)).into())
        }
        unsafe extern "system" fn GetPreset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpreset: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpreset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lquality: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuality(this, ::core::mem::transmute_copy(&lquality)).into())
        }
        unsafe extern "system" fn GetQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plquality: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQuality(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquality, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDirectSoundFXI3DL2Reverb_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            SetPreset: SetPreset::<Identity, Impl, OFFSET>,
            GetPreset: GetPreset::<Identity, Impl, OFFSET>,
            SetQuality: SetQuality::<Identity, Impl, OFFSET>,
            GetQuality: GetQuality::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundFXParamEq_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdsfxparameq: *const DSFXParamEq) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This) -> ::windows_core::Result<DSFXParamEq>;
}
impl ::windows_core::Iids for IDirectSoundFXParamEq {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXParamEq_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFXParamEq {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXParamEq_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxparameq: *const DSFXParamEq) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdsfxparameq)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXParamEq_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxparameq: *mut DSFXParamEq) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdsfxparameq, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDirectSoundFXParamEq_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDirectSoundFXWavesReverb_Impl: ::windows_core::BaseImpl {
    fn SetAllParameters(this: &Self::This, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows_core::Result<()>;
    fn GetAllParameters(this: &Self::This) -> ::windows_core::Result<DSFXWavesReverb>;
}
impl ::windows_core::Iids for IDirectSoundFXWavesReverb {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXWavesReverb_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFXWavesReverb {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXWavesReverb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllParameters(this, ::core::mem::transmute_copy(&pcdsfxwavesreverb)).into())
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFXWavesReverb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdsfxwavesreverb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDirectSoundFXWavesReverb_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundFullDuplex_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pcaptureguid: *const ::windows_core::GUID, prenderguid: *const ::windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::core::option::Option<IDirectSoundCaptureBuffer8>, lplpdirectsoundbuffer8: *mut ::core::option::Option<IDirectSoundBuffer8>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectSoundFullDuplex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFullDuplex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundFullDuplex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundFullDuplex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaptureguid: *const ::windows_core::GUID, prenderguid: *const ::windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut *mut ::core::ffi::c_void, lplpdirectsoundbuffer8: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&pcaptureguid), ::core::mem::transmute_copy(&prenderguid), ::core::mem::transmute_copy(&lpdscbufferdesc), ::core::mem::transmute_copy(&lpdsbufferdesc), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&lplpdirectsoundcapturebuffer8), ::core::mem::transmute_copy(&lplpdirectsoundbuffer8)).into())
        }
        IDirectSoundFullDuplex_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundNotify_Impl: ::windows_core::BaseImpl {
    fn SetNotificationPositions(this: &Self::This, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectSoundNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectSoundNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetNotificationPositions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectSoundNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotificationPositions(this, ::core::mem::transmute_copy(&dwpositionnotifies), ::core::mem::transmute_copy(&pcpositionnotifies)).into())
        }
        IDirectSoundNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetNotificationPositions: SetNotificationPositions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
