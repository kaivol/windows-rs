#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait IMediaEnginePlaybackSource_Impl: ::windows_core::BaseImpl {
    fn CurrentItem(this: &Self::This) -> ::windows_core::Result<MediaPlaybackItem>;
    fn SetPlaybackSource(this: &Self::This, source: ::core::option::Option<&IMediaPlaybackSource>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for IMediaEnginePlaybackSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaEnginePlaybackSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlaybackSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlaybackSource(this, ::windows_core::from_raw_borrowed(&source)).into())
        }
        IMediaEnginePlaybackSource_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentItem: CurrentItem::<Identity, Impl, OFFSET>,
            SetPlaybackSource: SetPlaybackSource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMediaPlaybackSource_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IMediaPlaybackSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaPlaybackSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaPlaybackSource {
    const VTABLE: Self::Vtable = { IMediaPlaybackSource_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
