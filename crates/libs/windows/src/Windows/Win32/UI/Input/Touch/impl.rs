#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInertiaProcessor_Impl: ::windows_core::BaseImpl {
    fn InitialOriginX(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetInitialOriginX(this: &Self::This, x: f32) -> ::windows_core::Result<()>;
    fn InitialOriginY(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetInitialOriginY(this: &Self::This, y: f32) -> ::windows_core::Result<()>;
    fn InitialVelocityX(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetInitialVelocityX(this: &Self::This, x: f32) -> ::windows_core::Result<()>;
    fn InitialVelocityY(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetInitialVelocityY(this: &Self::This, y: f32) -> ::windows_core::Result<()>;
    fn InitialAngularVelocity(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetInitialAngularVelocity(this: &Self::This, velocity: f32) -> ::windows_core::Result<()>;
    fn InitialExpansionVelocity(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetInitialExpansionVelocity(this: &Self::This, velocity: f32) -> ::windows_core::Result<()>;
    fn InitialRadius(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetInitialRadius(this: &Self::This, radius: f32) -> ::windows_core::Result<()>;
    fn BoundaryLeft(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetBoundaryLeft(this: &Self::This, left: f32) -> ::windows_core::Result<()>;
    fn BoundaryTop(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetBoundaryTop(this: &Self::This, top: f32) -> ::windows_core::Result<()>;
    fn BoundaryRight(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetBoundaryRight(this: &Self::This, right: f32) -> ::windows_core::Result<()>;
    fn BoundaryBottom(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetBoundaryBottom(this: &Self::This, bottom: f32) -> ::windows_core::Result<()>;
    fn ElasticMarginLeft(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetElasticMarginLeft(this: &Self::This, left: f32) -> ::windows_core::Result<()>;
    fn ElasticMarginTop(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetElasticMarginTop(this: &Self::This, top: f32) -> ::windows_core::Result<()>;
    fn ElasticMarginRight(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetElasticMarginRight(this: &Self::This, right: f32) -> ::windows_core::Result<()>;
    fn ElasticMarginBottom(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetElasticMarginBottom(this: &Self::This, bottom: f32) -> ::windows_core::Result<()>;
    fn DesiredDisplacement(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetDesiredDisplacement(this: &Self::This, displacement: f32) -> ::windows_core::Result<()>;
    fn DesiredRotation(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetDesiredRotation(this: &Self::This, rotation: f32) -> ::windows_core::Result<()>;
    fn DesiredExpansion(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetDesiredExpansion(this: &Self::This, expansion: f32) -> ::windows_core::Result<()>;
    fn DesiredDeceleration(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetDesiredDeceleration(this: &Self::This, deceleration: f32) -> ::windows_core::Result<()>;
    fn DesiredAngularDeceleration(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetDesiredAngularDeceleration(this: &Self::This, deceleration: f32) -> ::windows_core::Result<()>;
    fn DesiredExpansionDeceleration(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetDesiredExpansionDeceleration(this: &Self::This, deceleration: f32) -> ::windows_core::Result<()>;
    fn InitialTimestamp(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetInitialTimestamp(this: &Self::This, timestamp: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Process(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn ProcessTime(this: &Self::This, timestamp: u32) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn Complete(this: &Self::This) -> ::windows_core::Result<()>;
    fn CompleteTime(this: &Self::This, timestamp: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInertiaProcessor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInertiaProcessor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitialOriginX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialOriginX(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(x, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialOriginX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialOriginX(this, ::core::mem::transmute_copy(&x)).into())
        }
        unsafe extern "system" fn InitialOriginY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialOriginY(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(y, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialOriginY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialOriginY(this, ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn InitialVelocityX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialVelocityX(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(x, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialVelocityX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialVelocityX(this, ::core::mem::transmute_copy(&x)).into())
        }
        unsafe extern "system" fn InitialVelocityY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialVelocityY(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(y, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialVelocityY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, y: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialVelocityY(this, ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn InitialAngularVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialAngularVelocity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(velocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialAngularVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialAngularVelocity(this, ::core::mem::transmute_copy(&velocity)).into())
        }
        unsafe extern "system" fn InitialExpansionVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialExpansionVelocity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(velocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialExpansionVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialExpansionVelocity(this, ::core::mem::transmute_copy(&velocity)).into())
        }
        unsafe extern "system" fn InitialRadius<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialRadius(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(radius, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialRadius<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialRadius(this, ::core::mem::transmute_copy(&radius)).into())
        }
        unsafe extern "system" fn BoundaryLeft<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BoundaryLeft(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(left, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBoundaryLeft<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBoundaryLeft(this, ::core::mem::transmute_copy(&left)).into())
        }
        unsafe extern "system" fn BoundaryTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BoundaryTop(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(top, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBoundaryTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBoundaryTop(this, ::core::mem::transmute_copy(&top)).into())
        }
        unsafe extern "system" fn BoundaryRight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BoundaryRight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(right, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBoundaryRight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBoundaryRight(this, ::core::mem::transmute_copy(&right)).into())
        }
        unsafe extern "system" fn BoundaryBottom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BoundaryBottom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bottom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBoundaryBottom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBoundaryBottom(this, ::core::mem::transmute_copy(&bottom)).into())
        }
        unsafe extern "system" fn ElasticMarginLeft<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElasticMarginLeft(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(left, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetElasticMarginLeft<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetElasticMarginLeft(this, ::core::mem::transmute_copy(&left)).into())
        }
        unsafe extern "system" fn ElasticMarginTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElasticMarginTop(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(top, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetElasticMarginTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetElasticMarginTop(this, ::core::mem::transmute_copy(&top)).into())
        }
        unsafe extern "system" fn ElasticMarginRight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElasticMarginRight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(right, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetElasticMarginRight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetElasticMarginRight(this, ::core::mem::transmute_copy(&right)).into())
        }
        unsafe extern "system" fn ElasticMarginBottom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElasticMarginBottom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bottom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetElasticMarginBottom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetElasticMarginBottom(this, ::core::mem::transmute_copy(&bottom)).into())
        }
        unsafe extern "system" fn DesiredDisplacement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displacement: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DesiredDisplacement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displacement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredDisplacement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displacement: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesiredDisplacement(this, ::core::mem::transmute_copy(&displacement)).into())
        }
        unsafe extern "system" fn DesiredRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rotation: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DesiredRotation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rotation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rotation: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesiredRotation(this, ::core::mem::transmute_copy(&rotation)).into())
        }
        unsafe extern "system" fn DesiredExpansion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expansion: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DesiredExpansion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expansion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredExpansion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expansion: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesiredExpansion(this, ::core::mem::transmute_copy(&expansion)).into())
        }
        unsafe extern "system" fn DesiredDeceleration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DesiredDeceleration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deceleration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredDeceleration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesiredDeceleration(this, ::core::mem::transmute_copy(&deceleration)).into())
        }
        unsafe extern "system" fn DesiredAngularDeceleration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DesiredAngularDeceleration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deceleration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredAngularDeceleration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesiredAngularDeceleration(this, ::core::mem::transmute_copy(&deceleration)).into())
        }
        unsafe extern "system" fn DesiredExpansionDeceleration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DesiredExpansionDeceleration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deceleration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredExpansionDeceleration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesiredExpansionDeceleration(this, ::core::mem::transmute_copy(&deceleration)).into())
        }
        unsafe extern "system" fn InitialTimestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialTimestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timestamp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialTimestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialTimestamp(this, ::core::mem::transmute_copy(&timestamp)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Process<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Process(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProcessTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProcessTime(this, ::core::mem::transmute_copy(&timestamp)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Complete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Complete(this).into())
        }
        unsafe extern "system" fn CompleteTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInertiaProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompleteTime(this, ::core::mem::transmute_copy(&timestamp)).into())
        }
        IInertiaProcessor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitialOriginX: InitialOriginX::<Identity, Impl, OFFSET>,
            SetInitialOriginX: SetInitialOriginX::<Identity, Impl, OFFSET>,
            InitialOriginY: InitialOriginY::<Identity, Impl, OFFSET>,
            SetInitialOriginY: SetInitialOriginY::<Identity, Impl, OFFSET>,
            InitialVelocityX: InitialVelocityX::<Identity, Impl, OFFSET>,
            SetInitialVelocityX: SetInitialVelocityX::<Identity, Impl, OFFSET>,
            InitialVelocityY: InitialVelocityY::<Identity, Impl, OFFSET>,
            SetInitialVelocityY: SetInitialVelocityY::<Identity, Impl, OFFSET>,
            InitialAngularVelocity: InitialAngularVelocity::<Identity, Impl, OFFSET>,
            SetInitialAngularVelocity: SetInitialAngularVelocity::<Identity, Impl, OFFSET>,
            InitialExpansionVelocity: InitialExpansionVelocity::<Identity, Impl, OFFSET>,
            SetInitialExpansionVelocity: SetInitialExpansionVelocity::<Identity, Impl, OFFSET>,
            InitialRadius: InitialRadius::<Identity, Impl, OFFSET>,
            SetInitialRadius: SetInitialRadius::<Identity, Impl, OFFSET>,
            BoundaryLeft: BoundaryLeft::<Identity, Impl, OFFSET>,
            SetBoundaryLeft: SetBoundaryLeft::<Identity, Impl, OFFSET>,
            BoundaryTop: BoundaryTop::<Identity, Impl, OFFSET>,
            SetBoundaryTop: SetBoundaryTop::<Identity, Impl, OFFSET>,
            BoundaryRight: BoundaryRight::<Identity, Impl, OFFSET>,
            SetBoundaryRight: SetBoundaryRight::<Identity, Impl, OFFSET>,
            BoundaryBottom: BoundaryBottom::<Identity, Impl, OFFSET>,
            SetBoundaryBottom: SetBoundaryBottom::<Identity, Impl, OFFSET>,
            ElasticMarginLeft: ElasticMarginLeft::<Identity, Impl, OFFSET>,
            SetElasticMarginLeft: SetElasticMarginLeft::<Identity, Impl, OFFSET>,
            ElasticMarginTop: ElasticMarginTop::<Identity, Impl, OFFSET>,
            SetElasticMarginTop: SetElasticMarginTop::<Identity, Impl, OFFSET>,
            ElasticMarginRight: ElasticMarginRight::<Identity, Impl, OFFSET>,
            SetElasticMarginRight: SetElasticMarginRight::<Identity, Impl, OFFSET>,
            ElasticMarginBottom: ElasticMarginBottom::<Identity, Impl, OFFSET>,
            SetElasticMarginBottom: SetElasticMarginBottom::<Identity, Impl, OFFSET>,
            DesiredDisplacement: DesiredDisplacement::<Identity, Impl, OFFSET>,
            SetDesiredDisplacement: SetDesiredDisplacement::<Identity, Impl, OFFSET>,
            DesiredRotation: DesiredRotation::<Identity, Impl, OFFSET>,
            SetDesiredRotation: SetDesiredRotation::<Identity, Impl, OFFSET>,
            DesiredExpansion: DesiredExpansion::<Identity, Impl, OFFSET>,
            SetDesiredExpansion: SetDesiredExpansion::<Identity, Impl, OFFSET>,
            DesiredDeceleration: DesiredDeceleration::<Identity, Impl, OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Identity, Impl, OFFSET>,
            DesiredAngularDeceleration: DesiredAngularDeceleration::<Identity, Impl, OFFSET>,
            SetDesiredAngularDeceleration: SetDesiredAngularDeceleration::<Identity, Impl, OFFSET>,
            DesiredExpansionDeceleration: DesiredExpansionDeceleration::<Identity, Impl, OFFSET>,
            SetDesiredExpansionDeceleration: SetDesiredExpansionDeceleration::<Identity, Impl, OFFSET>,
            InitialTimestamp: InitialTimestamp::<Identity, Impl, OFFSET>,
            SetInitialTimestamp: SetInitialTimestamp::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Process: Process::<Identity, Impl, OFFSET>,
            ProcessTime: ProcessTime::<Identity, Impl, OFFSET>,
            Complete: Complete::<Identity, Impl, OFFSET>,
            CompleteTime: CompleteTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IManipulationProcessor_Impl: ::windows_core::BaseImpl {
    fn SupportedManipulations(this: &Self::This) -> ::windows_core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS>;
    fn SetSupportedManipulations(this: &Self::This, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows_core::Result<()>;
    fn PivotPointX(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetPivotPointX(this: &Self::This, pivotpointx: f32) -> ::windows_core::Result<()>;
    fn PivotPointY(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetPivotPointY(this: &Self::This, pivotpointy: f32) -> ::windows_core::Result<()>;
    fn PivotRadius(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetPivotRadius(this: &Self::This, pivotradius: f32) -> ::windows_core::Result<()>;
    fn CompleteManipulation(this: &Self::This) -> ::windows_core::Result<()>;
    fn ProcessDown(this: &Self::This, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn ProcessMove(this: &Self::This, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn ProcessUp(this: &Self::This, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn ProcessDownWithTime(this: &Self::This, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::Result<()>;
    fn ProcessMoveWithTime(this: &Self::This, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::Result<()>;
    fn ProcessUpWithTime(this: &Self::This, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::Result<()>;
    fn GetVelocityX(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetVelocityY(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetExpansionVelocity(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetAngularVelocity(this: &Self::This) -> ::windows_core::Result<f32>;
    fn MinimumScaleRotateRadius(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetMinimumScaleRotateRadius(this: &Self::This, minradius: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IManipulationProcessor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IManipulationProcessor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SupportedManipulations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedManipulations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manipulations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSupportedManipulations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSupportedManipulations(this, ::core::mem::transmute_copy(&manipulations)).into())
        }
        unsafe extern "system" fn PivotPointX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pivotpointx: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PivotPointX(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pivotpointx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPivotPointX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pivotpointx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPivotPointX(this, ::core::mem::transmute_copy(&pivotpointx)).into())
        }
        unsafe extern "system" fn PivotPointY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pivotpointy: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PivotPointY(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pivotpointy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPivotPointY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pivotpointy: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPivotPointY(this, ::core::mem::transmute_copy(&pivotpointy)).into())
        }
        unsafe extern "system" fn PivotRadius<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pivotradius: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PivotRadius(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pivotradius, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPivotRadius<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pivotradius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPivotRadius(this, ::core::mem::transmute_copy(&pivotradius)).into())
        }
        unsafe extern "system" fn CompleteManipulation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompleteManipulation(this).into())
        }
        unsafe extern "system" fn ProcessDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessDown(this, ::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn ProcessMove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessMove(this, ::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn ProcessUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessUp(this, ::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn ProcessDownWithTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessDownWithTime(this, ::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into())
        }
        unsafe extern "system" fn ProcessMoveWithTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessMoveWithTime(this, ::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into())
        }
        unsafe extern "system" fn ProcessUpWithTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessUpWithTime(this, ::core::mem::transmute_copy(&manipulatorid), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&timestamp)).into())
        }
        unsafe extern "system" fn GetVelocityX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocityx: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVelocityX(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(velocityx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVelocityY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, velocityy: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVelocityY(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(velocityy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetExpansionVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expansionvelocity: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExpansionVelocity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expansionvelocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAngularVelocity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, angularvelocity: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAngularVelocity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(angularvelocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinimumScaleRotateRadius<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minradius: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinimumScaleRotateRadius(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minradius, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinimumScaleRotateRadius<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManipulationProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minradius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinimumScaleRotateRadius(this, ::core::mem::transmute_copy(&minradius)).into())
        }
        IManipulationProcessor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SupportedManipulations: SupportedManipulations::<Identity, Impl, OFFSET>,
            SetSupportedManipulations: SetSupportedManipulations::<Identity, Impl, OFFSET>,
            PivotPointX: PivotPointX::<Identity, Impl, OFFSET>,
            SetPivotPointX: SetPivotPointX::<Identity, Impl, OFFSET>,
            PivotPointY: PivotPointY::<Identity, Impl, OFFSET>,
            SetPivotPointY: SetPivotPointY::<Identity, Impl, OFFSET>,
            PivotRadius: PivotRadius::<Identity, Impl, OFFSET>,
            SetPivotRadius: SetPivotRadius::<Identity, Impl, OFFSET>,
            CompleteManipulation: CompleteManipulation::<Identity, Impl, OFFSET>,
            ProcessDown: ProcessDown::<Identity, Impl, OFFSET>,
            ProcessMove: ProcessMove::<Identity, Impl, OFFSET>,
            ProcessUp: ProcessUp::<Identity, Impl, OFFSET>,
            ProcessDownWithTime: ProcessDownWithTime::<Identity, Impl, OFFSET>,
            ProcessMoveWithTime: ProcessMoveWithTime::<Identity, Impl, OFFSET>,
            ProcessUpWithTime: ProcessUpWithTime::<Identity, Impl, OFFSET>,
            GetVelocityX: GetVelocityX::<Identity, Impl, OFFSET>,
            GetVelocityY: GetVelocityY::<Identity, Impl, OFFSET>,
            GetExpansionVelocity: GetExpansionVelocity::<Identity, Impl, OFFSET>,
            GetAngularVelocity: GetAngularVelocity::<Identity, Impl, OFFSET>,
            MinimumScaleRotateRadius: MinimumScaleRotateRadius::<Identity, Impl, OFFSET>,
            SetMinimumScaleRotateRadius: SetMinimumScaleRotateRadius::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait _IManipulationEvents_Impl: ::windows_core::BaseImpl {
    fn ManipulationStarted(this: &Self::This, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn ManipulationDelta(this: &Self::This, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_core::Result<()>;
    fn ManipulationCompleted(this: &Self::This, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for _IManipulationEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _IManipulationEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _IManipulationEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ManipulationStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _IManipulationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ManipulationStarted(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn ManipulationDelta<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _IManipulationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::ManipulationDelta(
                    this,
                    ::core::mem::transmute_copy(&x),
                    ::core::mem::transmute_copy(&y),
                    ::core::mem::transmute_copy(&translationdeltax),
                    ::core::mem::transmute_copy(&translationdeltay),
                    ::core::mem::transmute_copy(&scaledelta),
                    ::core::mem::transmute_copy(&expansiondelta),
                    ::core::mem::transmute_copy(&rotationdelta),
                    ::core::mem::transmute_copy(&cumulativetranslationx),
                    ::core::mem::transmute_copy(&cumulativetranslationy),
                    ::core::mem::transmute_copy(&cumulativescale),
                    ::core::mem::transmute_copy(&cumulativeexpansion),
                    ::core::mem::transmute_copy(&cumulativerotation),
                )
                .into()
            })
        }
        unsafe extern "system" fn ManipulationCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _IManipulationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ManipulationCompleted(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cumulativetranslationx), ::core::mem::transmute_copy(&cumulativetranslationy), ::core::mem::transmute_copy(&cumulativescale), ::core::mem::transmute_copy(&cumulativeexpansion), ::core::mem::transmute_copy(&cumulativerotation)).into())
        }
        _IManipulationEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ManipulationStarted: ManipulationStarted::<Identity, Impl, OFFSET>,
            ManipulationDelta: ManipulationDelta::<Identity, Impl, OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
