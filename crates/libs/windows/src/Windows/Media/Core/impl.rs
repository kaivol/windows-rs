#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IMediaCue_Impl: ::windows_core::BaseImpl {
    fn SetStartTime(this: &Self::This, value: &super::super::Foundation::TimeSpan) -> ::windows_core::Result<()>;
    fn StartTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(this: &Self::This, value: &super::super::Foundation::TimeSpan) -> ::windows_core::Result<()>;
    fn Duration(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::TimeSpan>;
    fn SetId(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IMediaCue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaCue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartTime(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDuration(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Duration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Duration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaCue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaCue_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMediaSource_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IMediaSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaSource {
    const VTABLE: Self::Vtable = { IMediaSource_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMediaStreamDescriptor_Impl: ::windows_core::BaseImpl {
    fn IsSelected(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetName(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetLanguage(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Language(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IMediaStreamDescriptor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaStreamDescriptor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSelected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguage(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaStreamDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Language(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaStreamDescriptor_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMediaStreamDescriptor2_Impl: ::windows_core::BaseImpl + IMediaStreamDescriptor_Impl {
    fn SetLabel(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IMediaStreamDescriptor2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IMediaStreamDescriptor as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaStreamDescriptor2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaStreamDescriptor2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaStreamDescriptor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaStreamDescriptor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaStreamDescriptor2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMediaTrack_Impl: ::windows_core::BaseImpl {
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Language(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn TrackKind(this: &Self::This) -> ::windows_core::Result<MediaTrackKind>;
    fn SetLabel(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IMediaTrack {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaTrack {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Language(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TrackKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaTrackKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrackKind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaTrack_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            TrackKind: TrackKind::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ISingleSelectMediaTrackList_Impl: ::windows_core::BaseImpl {
    fn SelectedIndexChanged(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectedIndexChanged(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn SetSelectedIndex(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn SelectedIndex(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ISingleSelectMediaTrackList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISingleSelectMediaTrackList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SelectedIndexChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectedIndexChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveSelectedIndexChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSelectedIndexChanged(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn SetSelectedIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelectedIndex(this, value).into())
        }
        unsafe extern "system" fn SelectedIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISingleSelectMediaTrackList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectedIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISingleSelectMediaTrackList_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SelectedIndexChanged: SelectedIndexChanged::<Identity, Impl, OFFSET>,
            RemoveSelectedIndexChanged: RemoveSelectedIndexChanged::<Identity, Impl, OFFSET>,
            SetSelectedIndex: SetSelectedIndex::<Identity, Impl, OFFSET>,
            SelectedIndex: SelectedIndex::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait ITimedMetadataTrackProvider_Impl: ::windows_core::BaseImpl {
    fn TimedMetadataTracks(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for ITimedMetadataTrackProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimedMetadataTrackProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITimedMetadataTrackProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TimedMetadataTracks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimedMetadataTrackProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TimedMetadataTracks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITimedMetadataTrackProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TimedMetadataTracks: TimedMetadataTracks::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
