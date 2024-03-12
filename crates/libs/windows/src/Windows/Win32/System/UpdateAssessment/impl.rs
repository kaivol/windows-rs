#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWaaSAssessor_Impl: ::windows_core::BaseImpl {
    fn GetOSUpdateAssessment(this: &Self::This) -> ::windows_core::Result<OSUpdateAssessment>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWaaSAssessor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWaaSAssessor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWaaSAssessor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOSUpdateAssessment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWaaSAssessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result: *mut OSUpdateAssessment) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOSUpdateAssessment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWaaSAssessor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOSUpdateAssessment: GetOSUpdateAssessment::<Identity, Impl, OFFSET>,
        }
    };
}
