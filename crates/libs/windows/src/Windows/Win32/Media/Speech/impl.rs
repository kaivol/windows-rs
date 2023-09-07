pub trait IEnumSpObjectTokens_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, pelt: *mut ::core::option::Option<ISpObjectToken>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSpObjectTokens>;
    fn Item(this: &Self::This, index: u32) -> ::windows_core::Result<ISpObjectToken>;
    fn GetCount(this: &Self::This, pcount: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumSpObjectTokens {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSpObjectTokens {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this, ::core::mem::transmute_copy(&pcount)).into())
        }
        IEnumSpObjectTokens_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpAudio_Impl: ::windows_core::BaseImpl + ISpStreamFormat_Impl {
    fn SetState(this: &Self::This, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows_core::Result<()>;
    fn SetFormat(this: &Self::This, rguidfmtid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This, pstatus: *mut SPAUDIOSTATUS) -> ::windows_core::Result<()>;
    fn SetBufferInfo(this: &Self::This, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows_core::Result<()>;
    fn GetBufferInfo(this: &Self::This, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows_core::Result<()>;
    fn GetDefaultFormat(this: &Self::This, pformatid: *mut ::windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn EventHandle(this: &Self::This) -> super::super::Foundation::HANDLE;
    fn GetVolumeLevel(this: &Self::This, plevel: *mut u32) -> ::windows_core::Result<()>;
    fn SetVolumeLevel(this: &Self::This, level: u32) -> ::windows_core::Result<()>;
    fn GetBufferNotifySize(this: &Self::This, pcbsize: *mut u32) -> ::windows_core::Result<()>;
    fn SetBufferNotifySize(this: &Self::This, cbsize: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpAudio {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpStreamFormat);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpAudio {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&newstate), ::core::mem::transmute_copy(&ullreserved)).into())
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidfmtid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormat(this, ::core::mem::transmute_copy(&rguidfmtid), ::core::mem::transmute_copy(&pwaveformatex)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPAUDIOSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pstatus)).into())
        }
        unsafe extern "system" fn SetBufferInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferInfo(this, ::core::mem::transmute_copy(&pbuffinfo)).into())
        }
        unsafe extern "system" fn GetBufferInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferInfo(this, ::core::mem::transmute_copy(&pbuffinfo)).into())
        }
        unsafe extern "system" fn GetDefaultFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatid: *mut ::windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultFormat(this, ::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&ppcomemwaveformatex)).into())
        }
        unsafe extern "system" fn EventHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EventHandle(this))
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVolumeLevel(this, ::core::mem::transmute_copy(&plevel)).into())
        }
        unsafe extern "system" fn SetVolumeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolumeLevel(this, ::core::mem::transmute_copy(&level)).into())
        }
        unsafe extern "system" fn GetBufferNotifySize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferNotifySize(this, ::core::mem::transmute_copy(&pcbsize)).into())
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferNotifySize(this, ::core::mem::transmute_copy(&cbsize)).into())
        }
        ISpAudio_Vtbl {
            base__: <ISpStreamFormat as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetState: SetState::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetBufferInfo: SetBufferInfo::<Identity, Impl, OFFSET>,
            GetBufferInfo: GetBufferInfo::<Identity, Impl, OFFSET>,
            GetDefaultFormat: GetDefaultFormat::<Identity, Impl, OFFSET>,
            EventHandle: EventHandle::<Identity, Impl, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, Impl, OFFSET>,
            SetVolumeLevel: SetVolumeLevel::<Identity, Impl, OFFSET>,
            GetBufferNotifySize: GetBufferNotifySize::<Identity, Impl, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpContainerLexicon_Impl: ::windows_core::BaseImpl + ISpLexicon_Impl {
    fn AddLexicon(this: &Self::This, paddlexicon: ::core::option::Option<&ISpLexicon>, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpContainerLexicon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpLexicon);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpContainerLexicon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpContainerLexicon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddLexicon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpContainerLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddlexicon: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLexicon(this, ::windows_core::from_raw_borrowed(&paddlexicon), ::core::mem::transmute_copy(&dwflags)).into())
        }
        ISpContainerLexicon_Vtbl { base__: <ISpLexicon as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AddLexicon: AddLexicon::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpDataKey_Impl: ::windows_core::BaseImpl {
    fn SetData(this: &Self::This, pszvaluename: &::windows_core::PCWSTR, cbdata: u32, pdata: *const u8) -> ::windows_core::Result<()>;
    fn GetData(this: &Self::This, pszvaluename: &::windows_core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows_core::Result<()>;
    fn SetStringValue(this: &Self::This, pszvaluename: &::windows_core::PCWSTR, pszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetStringValue(this: &Self::This, pszvaluename: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDWORD(this: &Self::This, pszvaluename: &::windows_core::PCWSTR, dwvalue: u32) -> ::windows_core::Result<()>;
    fn GetDWORD(this: &Self::This, pszvaluename: &::windows_core::PCWSTR, pdwvalue: *mut u32) -> ::windows_core::Result<()>;
    fn OpenKey(this: &Self::This, pszsubkeyname: &::windows_core::PCWSTR) -> ::windows_core::Result<ISpDataKey>;
    fn CreateKey(this: &Self::This, pszsubkey: &::windows_core::PCWSTR) -> ::windows_core::Result<ISpDataKey>;
    fn DeleteKey(this: &Self::This, pszsubkey: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DeleteValue(this: &Self::This, pszvaluename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumKeys(this: &Self::This, index: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn EnumValues(this: &Self::This, index: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for ISpDataKey {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpDataKey {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, cbdata: u32, pdata: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, pszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStringValue(this, ::core::mem::transmute(&pszvaluename), ::core::mem::transmute(&pszvalue)).into())
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, ppszvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringValue(this, ::core::mem::transmute(&pszvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, dwvalue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDWORD(this, ::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&dwvalue)).into())
        }
        unsafe extern "system" fn GetDWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, pdwvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDWORD(this, ::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&pdwvalue)).into())
        }
        unsafe extern "system" fn OpenKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubkeyname: ::windows_core::PCWSTR, ppsubkey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenKey(this, ::core::mem::transmute(&pszsubkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubkey: ::windows_core::PCWSTR, ppsubkey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateKey(this, ::core::mem::transmute(&pszsubkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubkey: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteKey(this, ::core::mem::transmute(&pszsubkey)).into())
        }
        unsafe extern "system" fn DeleteValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteValue(this, ::core::mem::transmute(&pszvaluename)).into())
        }
        unsafe extern "system" fn EnumKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppszsubkeyname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumKeys(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszsubkeyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppszvaluename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumValues(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszvaluename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpDataKey_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetData: SetData::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            SetDWORD: SetDWORD::<Identity, Impl, OFFSET>,
            GetDWORD: GetDWORD::<Identity, Impl, OFFSET>,
            OpenKey: OpenKey::<Identity, Impl, OFFSET>,
            CreateKey: CreateKey::<Identity, Impl, OFFSET>,
            DeleteKey: DeleteKey::<Identity, Impl, OFFSET>,
            DeleteValue: DeleteValue::<Identity, Impl, OFFSET>,
            EnumKeys: EnumKeys::<Identity, Impl, OFFSET>,
            EnumValues: EnumValues::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpDisplayAlternates_Impl: ::windows_core::BaseImpl {
    fn GetDisplayAlternates(this: &Self::This, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn SetFullStopTrailSpace(this: &Self::This, ultrailspace: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpDisplayAlternates {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDisplayAlternates_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpDisplayAlternates {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDisplayAlternates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDisplayAlternates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayAlternates(this, ::core::mem::transmute_copy(&pphrase), ::core::mem::transmute_copy(&crequestcount), ::core::mem::transmute_copy(&ppcomemphrases), ::core::mem::transmute_copy(&pcphrasesreturned)).into())
        }
        unsafe extern "system" fn SetFullStopTrailSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpDisplayAlternates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ultrailspace: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFullStopTrailSpace(this, ::core::mem::transmute_copy(&ultrailspace)).into())
        }
        ISpDisplayAlternates_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDisplayAlternates: GetDisplayAlternates::<Identity, Impl, OFFSET>,
            SetFullStopTrailSpace: SetFullStopTrailSpace::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpEnginePronunciation_Impl: ::windows_core::BaseImpl {
    fn Normalize(this: &Self::This, pszword: &::windows_core::PCWSTR, pszleftcontext: &::windows_core::PCWSTR, pszrightcontext: &::windows_core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows_core::Result<()>;
    fn GetPronunciations(this: &Self::This, pszword: &::windows_core::PCWSTR, pszleftcontext: &::windows_core::PCWSTR, pszrightcontext: &::windows_core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpEnginePronunciation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEnginePronunciation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpEnginePronunciation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Normalize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEnginePronunciation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, pszleftcontext: ::windows_core::PCWSTR, pszrightcontext: ::windows_core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Normalize(this, ::core::mem::transmute(&pszword), ::core::mem::transmute(&pszleftcontext), ::core::mem::transmute(&pszrightcontext), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pnormalizationlist)).into())
        }
        unsafe extern "system" fn GetPronunciations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEnginePronunciation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, pszleftcontext: ::windows_core::PCWSTR, pszrightcontext: ::windows_core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPronunciations(this, ::core::mem::transmute(&pszword), ::core::mem::transmute(&pszleftcontext), ::core::mem::transmute(&pszrightcontext), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&penginepronunciationlist)).into())
        }
        ISpEnginePronunciation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Normalize: Normalize::<Identity, Impl, OFFSET>,
            GetPronunciations: GetPronunciations::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSink_Impl: ::windows_core::BaseImpl {
    fn AddEvents(this: &Self::This, peventarray: *const SPEVENT, ulcount: u32) -> ::windows_core::Result<()>;
    fn GetEventInterest(this: &Self::This, pulleventinterest: *mut u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventarray: *const SPEVENT, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddEvents(this, ::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&ulcount)).into())
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEventInterest(this, ::core::mem::transmute_copy(&pulleventinterest)).into())
        }
        ISpEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddEvents: AddEvents::<Identity, Impl, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSource_Impl: ::windows_core::BaseImpl + ISpNotifySource_Impl {
    fn SetInterest(this: &Self::This, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows_core::Result<()>;
    fn GetEvents(this: &Self::This, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetInfo(this: &Self::This, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpEventSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpNotifySource);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEventSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpEventSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInterest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEventSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterest(this, ::core::mem::transmute_copy(&ulleventinterest), ::core::mem::transmute_copy(&ullqueuedinterest)).into())
        }
        unsafe extern "system" fn GetEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEventSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEvents(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&pulfetched)).into())
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEventSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInfo(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        ISpEventSource_Vtbl {
            base__: <ISpNotifySource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInterest: SetInterest::<Identity, Impl, OFFSET>,
            GetEvents: GetEvents::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSource2_Impl: ::windows_core::BaseImpl + ISpEventSource_Impl {
    fn GetEventsEx(this: &Self::This, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpEventSource2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpEventSource);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEventSource2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpEventSource2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEventsEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpEventSource2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEventsEx(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&pulfetched)).into())
        }
        ISpEventSource2_Vtbl { base__: <ISpEventSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetEventsEx: GetEventsEx::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpGrammarBuilder_Impl: ::windows_core::BaseImpl {
    fn ResetGrammar(this: &Self::This, newlanguage: u16) -> ::windows_core::Result<()>;
    fn GetRule(this: &Self::This, pszrulename: &::windows_core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut SPSTATEHANDLE) -> ::windows_core::Result<()>;
    fn ClearRule(this: &Self::This, hstate: SPSTATEHANDLE) -> ::windows_core::Result<()>;
    fn CreateNewState(this: &Self::This, hstate: SPSTATEHANDLE, phstate: *mut SPSTATEHANDLE) -> ::windows_core::Result<()>;
    fn AddWordTransition(this: &Self::This, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: &::windows_core::PCWSTR, pszseparators: &::windows_core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_core::Result<()>;
    fn AddRuleTransition(this: &Self::This, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, hrule: SPSTATEHANDLE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_core::Result<()>;
    fn AddResource(this: &Self::This, hrulestate: SPSTATEHANDLE, pszresourcename: &::windows_core::PCWSTR, pszresourcevalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This, dwreserved: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpGrammarBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpGrammarBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResetGrammar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newlanguage: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetGrammar(this, ::core::mem::transmute_copy(&newlanguage)).into())
        }
        unsafe extern "system" fn GetRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrulename: ::windows_core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut SPSTATEHANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRule(this, ::core::mem::transmute(&pszrulename), ::core::mem::transmute_copy(&dwruleid), ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&fcreateifnotexist), ::core::mem::transmute_copy(&phinitialstate)).into())
        }
        unsafe extern "system" fn ClearRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hstate: SPSTATEHANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRule(this, ::core::mem::transmute_copy(&hstate)).into())
        }
        unsafe extern "system" fn CreateNewState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hstate: SPSTATEHANDLE, phstate: *mut SPSTATEHANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateNewState(this, ::core::mem::transmute_copy(&hstate), ::core::mem::transmute_copy(&phstate)).into())
        }
        unsafe extern "system" fn AddWordTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: ::windows_core::PCWSTR, pszseparators: ::windows_core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddWordTransition(this, ::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute(&psz), ::core::mem::transmute(&pszseparators), ::core::mem::transmute_copy(&ewordtype), ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&ppropinfo)).into())
        }
        unsafe extern "system" fn AddRuleTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, hrule: SPSTATEHANDLE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRuleTransition(this, ::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute_copy(&hrule), ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&ppropinfo)).into())
        }
        unsafe extern "system" fn AddResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrulestate: SPSTATEHANDLE, pszresourcename: ::windows_core::PCWSTR, pszresourcevalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddResource(this, ::core::mem::transmute_copy(&hrulestate), ::core::mem::transmute(&pszresourcename), ::core::mem::transmute(&pszresourcevalue)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&dwreserved)).into())
        }
        ISpGrammarBuilder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResetGrammar: ResetGrammar::<Identity, Impl, OFFSET>,
            GetRule: GetRule::<Identity, Impl, OFFSET>,
            ClearRule: ClearRule::<Identity, Impl, OFFSET>,
            CreateNewState: CreateNewState::<Identity, Impl, OFFSET>,
            AddWordTransition: AddWordTransition::<Identity, Impl, OFFSET>,
            AddRuleTransition: AddRuleTransition::<Identity, Impl, OFFSET>,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpGrammarBuilder2_Impl: ::windows_core::BaseImpl {
    fn AddTextSubset(this: &Self::This, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: &::windows_core::PCWSTR, ematchmode: SPMATCHINGMODE) -> ::windows_core::Result<()>;
    fn SetPhoneticAlphabet(this: &Self::This, phoneticalphabet: PHONETICALPHABET) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpGrammarBuilder2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpGrammarBuilder2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddTextSubset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: ::windows_core::PCWSTR, ematchmode: SPMATCHINGMODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTextSubset(this, ::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute(&psz), ::core::mem::transmute_copy(&ematchmode)).into())
        }
        unsafe extern "system" fn SetPhoneticAlphabet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpGrammarBuilder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phoneticalphabet: PHONETICALPHABET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPhoneticAlphabet(this, ::core::mem::transmute_copy(&phoneticalphabet)).into())
        }
        ISpGrammarBuilder2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddTextSubset: AddTextSubset::<Identity, Impl, OFFSET>,
            SetPhoneticAlphabet: SetPhoneticAlphabet::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpLexicon_Impl: ::windows_core::BaseImpl {
    fn GetPronunciations(this: &Self::This, pszword: &::windows_core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_core::Result<()>;
    fn AddPronunciation(this: &Self::This, pszword: &::windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_core::Result<()>;
    fn RemovePronunciation(this: &Self::This, pszword: &::windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_core::Result<()>;
    fn GetGeneration(this: &Self::This, pdwgeneration: *mut u32) -> ::windows_core::Result<()>;
    fn GetGenerationChange(this: &Self::This, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::Result<()>;
    fn GetWords(this: &Self::This, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpLexicon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpLexicon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPronunciations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPronunciations(this, ::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwordpronunciationlist)).into())
        }
        unsafe extern "system" fn AddPronunciation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPronunciation(this, ::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&epartofspeech), ::core::mem::transmute_copy(&pszpronunciation)).into())
        }
        unsafe extern "system" fn RemovePronunciation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePronunciation(this, ::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&epartofspeech), ::core::mem::transmute_copy(&pszpronunciation)).into())
        }
        unsafe extern "system" fn GetGeneration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGeneration(this, ::core::mem::transmute_copy(&pdwgeneration)).into())
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGenerationChange(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pwordlist)).into())
        }
        unsafe extern "system" fn GetWords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWords(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pwordlist)).into())
        }
        ISpLexicon_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPronunciations: GetPronunciations::<Identity, Impl, OFFSET>,
            AddPronunciation: AddPronunciation::<Identity, Impl, OFFSET>,
            RemovePronunciation: RemovePronunciation::<Identity, Impl, OFFSET>,
            GetGeneration: GetGeneration::<Identity, Impl, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpMMSysAudio_Impl: ::windows_core::BaseImpl + ISpAudio_Impl {
    fn GetDeviceId(this: &Self::This, pudeviceid: *mut u32) -> ::windows_core::Result<()>;
    fn SetDeviceId(this: &Self::This, udeviceid: u32) -> ::windows_core::Result<()>;
    fn GetMMHandle(this: &Self::This, phandle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetLineId(this: &Self::This, pulineid: *mut u32) -> ::windows_core::Result<()>;
    fn SetLineId(this: &Self::This, ulineid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpMMSysAudio {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpAudio);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpMMSysAudio {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pudeviceid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceId(this, ::core::mem::transmute_copy(&pudeviceid)).into())
        }
        unsafe extern "system" fn SetDeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, udeviceid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeviceId(this, ::core::mem::transmute_copy(&udeviceid)).into())
        }
        unsafe extern "system" fn GetMMHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMMHandle(this, ::core::mem::transmute_copy(&phandle)).into())
        }
        unsafe extern "system" fn GetLineId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulineid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLineId(this, ::core::mem::transmute_copy(&pulineid)).into())
        }
        unsafe extern "system" fn SetLineId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulineid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLineId(this, ::core::mem::transmute_copy(&ulineid)).into())
        }
        ISpMMSysAudio_Vtbl {
            base__: <ISpAudio as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceId: GetDeviceId::<Identity, Impl, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, Impl, OFFSET>,
            GetMMHandle: GetMMHandle::<Identity, Impl, OFFSET>,
            GetLineId: GetLineId::<Identity, Impl, OFFSET>,
            SetLineId: SetLineId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifyCallback_Impl: Sized {
    fn NotifyCallback(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifyCallback_Vtbl {
    pub const fn new<Impl: ISpNotifyCallback_Impl>() -> ISpNotifyCallback_Vtbl {
        unsafe extern "system" fn NotifyCallback<Impl: ISpNotifyCallback_Impl>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::NotifyCallback(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self { NotifyCallback: NotifyCallback::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ISpNotifyCallback_ImplVtbl<T: ISpNotifyCallback_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ISpNotifyCallback_Impl> ISpNotifyCallback_ImplVtbl<T> {
    const VTABLE: ISpNotifyCallback_Vtbl = ISpNotifyCallback_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifyCallback {
    pub fn new<'a, T: ISpNotifyCallback_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ISpNotifyCallback_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait ISpNotifySink_Impl: ::windows_core::BaseImpl {
    fn Notify(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this).into())
        }
        ISpNotifySink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Notify: Notify::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifySource_Impl: ::windows_core::BaseImpl {
    fn SetNotifySink(this: &Self::This, pnotifysink: ::core::option::Option<&ISpNotifySink>) -> ::windows_core::Result<()>;
    fn SetNotifyWindowMessage(this: &Self::This, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetNotifyCallbackFunction(this: &Self::This, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetNotifyCallbackInterface(this: &Self::This, pspcallback: ::core::option::Option<&ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetNotifyWin32Event(this: &Self::This) -> ::windows_core::Result<()>;
    fn WaitForNotifyEvent(this: &Self::This, dwmilliseconds: u32) -> ::windows_core::Result<()>;
    fn GetNotifyEventHandle(this: &Self::This) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpNotifySource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpNotifySource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetNotifySink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnotifysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotifySink(this, ::windows_core::from_raw_borrowed(&pnotifysink)).into())
        }
        unsafe extern "system" fn SetNotifyWindowMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotifyWindowMessage(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn SetNotifyCallbackFunction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotifyCallbackFunction(this, ::core::mem::transmute_copy(&pfncallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn SetNotifyCallbackInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pspcallback: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotifyCallbackInterface(this, ::windows_core::from_raw_borrowed(&pspcallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn SetNotifyWin32Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotifyWin32Event(this).into())
        }
        unsafe extern "system" fn WaitForNotifyEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForNotifyEvent(this, ::core::mem::transmute_copy(&dwmilliseconds)).into())
        }
        unsafe extern "system" fn GetNotifyEventHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNotifyEventHandle(this))
        }
        ISpNotifySource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetNotifySink: SetNotifySink::<Identity, Impl, OFFSET>,
            SetNotifyWindowMessage: SetNotifyWindowMessage::<Identity, Impl, OFFSET>,
            SetNotifyCallbackFunction: SetNotifyCallbackFunction::<Identity, Impl, OFFSET>,
            SetNotifyCallbackInterface: SetNotifyCallbackInterface::<Identity, Impl, OFFSET>,
            SetNotifyWin32Event: SetNotifyWin32Event::<Identity, Impl, OFFSET>,
            WaitForNotifyEvent: WaitForNotifyEvent::<Identity, Impl, OFFSET>,
            GetNotifyEventHandle: GetNotifyEventHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifyTranslator_Impl: ::windows_core::BaseImpl + ISpNotifySink_Impl {
    fn InitWindowMessage(this: &Self::This, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn InitCallback(this: &Self::This, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn InitSpNotifyCallback(this: &Self::This, pspcallback: ::core::option::Option<&ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn InitWin32Event(this: &Self::This, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Wait(this: &Self::This, dwmilliseconds: u32) -> ::windows_core::Result<()>;
    fn GetEventHandle(this: &Self::This) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpNotifyTranslator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpNotifySink);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpNotifyTranslator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitWindowMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitWindowMessage(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn InitCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitCallback(this, ::core::mem::transmute_copy(&pfncallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn InitSpNotifyCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pspcallback: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitSpNotifyCallback(this, ::windows_core::from_raw_borrowed(&pspcallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn InitWin32Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitWin32Event(this, ::core::mem::transmute_copy(&hevent), ::core::mem::transmute_copy(&fclosehandleonrelease)).into())
        }
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::core::mem::transmute_copy(&dwmilliseconds)).into())
        }
        unsafe extern "system" fn GetEventHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEventHandle(this))
        }
        ISpNotifyTranslator_Vtbl {
            base__: <ISpNotifySink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitWindowMessage: InitWindowMessage::<Identity, Impl, OFFSET>,
            InitCallback: InitCallback::<Identity, Impl, OFFSET>,
            InitSpNotifyCallback: InitSpNotifyCallback::<Identity, Impl, OFFSET>,
            InitWin32Event: InitWin32Event::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            GetEventHandle: GetEventHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectToken_Impl: ::windows_core::BaseImpl + ISpDataKey_Impl {
    fn SetId(this: &Self::This, pszcategoryid: &::windows_core::PCWSTR, psztokenid: &::windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetCategory(this: &Self::This) -> ::windows_core::Result<ISpObjectTokenCategory>;
    fn CreateInstance(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclscontext: u32, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetStorageFileName(this: &Self::This, clsidcaller: *const ::windows_core::GUID, pszvaluename: &::windows_core::PCWSTR, pszfilenamespecifier: &::windows_core::PCWSTR, nfolder: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn RemoveStorageFileName(this: &Self::This, clsidcaller: *const ::windows_core::GUID, pszkeyname: &::windows_core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, pclsidcaller: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn IsUISupported(this: &Self::This, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: ::core::option::Option<&::windows_core::IUnknown>, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DisplayUI(this: &Self::This, hwndparent: super::super::Foundation::HWND, psztitle: &::windows_core::PCWSTR, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn MatchesAttributes(this: &Self::This, pszattributes: &::windows_core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpObjectToken {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpDataKey);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpObjectToken {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows_core::PCWSTR, psztokenid: ::windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&pszcategoryid), ::core::mem::transmute(&psztokenid), ::core::mem::transmute_copy(&fcreateifnotexist)).into())
        }
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemtokenid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptokencategory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCategory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptokencategory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstance(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        unsafe extern "system" fn GetStorageFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows_core::GUID, pszvaluename: ::windows_core::PCWSTR, pszfilenamespecifier: ::windows_core::PCWSTR, nfolder: u32, ppszfilepath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStorageFileName(this, ::core::mem::transmute_copy(&clsidcaller), ::core::mem::transmute(&pszvaluename), ::core::mem::transmute(&pszfilenamespecifier), ::core::mem::transmute_copy(&nfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszfilepath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveStorageFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows_core::GUID, pszkeyname: ::windows_core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStorageFileName(this, ::core::mem::transmute_copy(&clsidcaller), ::core::mem::transmute(&pszkeyname), ::core::mem::transmute_copy(&fdeletefile)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsidcaller: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&pclsidcaller)).into())
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUISupported(this, ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::windows_core::from_raw_borrowed(&punkobject), ::core::mem::transmute_copy(&pfsupported)).into())
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows_core::PCWSTR, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayUI(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&psztitle), ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::windows_core::from_raw_borrowed(&punkobject)).into())
        }
        unsafe extern "system" fn MatchesAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszattributes: ::windows_core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MatchesAttributes(this, ::core::mem::transmute(&pszattributes), ::core::mem::transmute_copy(&pfmatches)).into())
        }
        ISpObjectToken_Vtbl {
            base__: <ISpDataKey as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            GetStorageFileName: GetStorageFileName::<Identity, Impl, OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            MatchesAttributes: MatchesAttributes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectTokenCategory_Impl: ::windows_core::BaseImpl + ISpDataKey_Impl {
    fn SetId(this: &Self::This, pszcategoryid: &::windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDataKey(this: &Self::This, spdkl: SPDATAKEYLOCATION) -> ::windows_core::Result<ISpDataKey>;
    fn EnumTokens(this: &Self::This, pzsreqattribs: &::windows_core::PCWSTR, pszoptattribs: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnumSpObjectTokens>;
    fn SetDefaultTokenId(this: &Self::This, psztokenid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDefaultTokenId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpObjectTokenCategory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpDataKey);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpObjectTokenCategory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&pszcategoryid), ::core::mem::transmute_copy(&fcreateifnotexist)).into())
        }
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcomemcategoryid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemcategoryid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spdkl: SPDATAKEYLOCATION, ppdatakey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataKey(this, ::core::mem::transmute_copy(&spdkl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatakey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumTokens<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pzsreqattribs: ::windows_core::PCWSTR, pszoptattribs: ::windows_core::PCWSTR, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumTokens(this, ::core::mem::transmute(&pzsreqattribs), ::core::mem::transmute(&pszoptattribs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultTokenId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztokenid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultTokenId(this, ::core::mem::transmute(&psztokenid)).into())
        }
        unsafe extern "system" fn GetDefaultTokenId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultTokenId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemtokenid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpObjectTokenCategory_Vtbl {
            base__: <ISpDataKey as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetDataKey: GetDataKey::<Identity, Impl, OFFSET>,
            EnumTokens: EnumTokens::<Identity, Impl, OFFSET>,
            SetDefaultTokenId: SetDefaultTokenId::<Identity, Impl, OFFSET>,
            GetDefaultTokenId: GetDefaultTokenId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectTokenInit_Impl: ::windows_core::BaseImpl + ISpObjectToken_Impl {
    fn InitFromDataKey(this: &Self::This, pszcategoryid: &::windows_core::PCWSTR, psztokenid: &::windows_core::PCWSTR, pdatakey: ::core::option::Option<&ISpDataKey>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpObjectTokenInit {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpObjectToken);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectTokenInit_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpObjectTokenInit {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitFromDataKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectTokenInit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows_core::PCWSTR, psztokenid: ::windows_core::PCWSTR, pdatakey: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitFromDataKey(this, ::core::mem::transmute(&pszcategoryid), ::core::mem::transmute(&psztokenid), ::windows_core::from_raw_borrowed(&pdatakey)).into())
        }
        ISpObjectTokenInit_Vtbl { base__: <ISpObjectToken as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, InitFromDataKey: InitFromDataKey::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpObjectWithToken_Impl: ::windows_core::BaseImpl {
    fn SetObjectToken(this: &Self::This, ptoken: ::core::option::Option<&ISpObjectToken>) -> ::windows_core::Result<()>;
    fn GetObjectToken(this: &Self::This) -> ::windows_core::Result<ISpObjectToken>;
}
impl ::windows_core::Iids for ISpObjectWithToken {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectWithToken_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpObjectWithToken {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetObjectToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectWithToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectToken(this, ::windows_core::from_raw_borrowed(&ptoken)).into())
        }
        unsafe extern "system" fn GetObjectToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpObjectWithToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectToken(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpObjectWithToken_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetObjectToken: SetObjectToken::<Identity, Impl, OFFSET>,
            GetObjectToken: GetObjectToken::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpPhoneConverter_Impl: ::windows_core::BaseImpl + ISpObjectWithToken_Impl {
    fn PhoneToId(this: &Self::This, pszphone: &::windows_core::PCWSTR) -> ::windows_core::Result<u16>;
    fn IdToPhone(this: &Self::This, pid: *const u16, pszphone: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpPhoneConverter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpObjectWithToken);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneConverter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpPhoneConverter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PhoneToId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszphone: ::windows_core::PCWSTR, pid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PhoneToId(this, ::core::mem::transmute(&pszphone)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IdToPhone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *const u16, pszphone: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IdToPhone(this, ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pszphone)).into())
        }
        ISpPhoneConverter_Vtbl {
            base__: <ISpObjectWithToken as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PhoneToId: PhoneToId::<Identity, Impl, OFFSET>,
            IdToPhone: IdToPhone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpPhoneticAlphabetConverter_Impl: ::windows_core::BaseImpl {
    fn GetLangId(this: &Self::This) -> ::windows_core::Result<u16>;
    fn SetLangId(this: &Self::This, langid: u16) -> ::windows_core::Result<()>;
    fn SAPI2UPS(this: &Self::This, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows_core::Result<()>;
    fn UPS2SAPI(this: &Self::This, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows_core::Result<()>;
    fn GetMaxConvertLength(this: &Self::This, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpPhoneticAlphabetConverter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpPhoneticAlphabetConverter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLangId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLangId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plangid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLangId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLangId(this, ::core::mem::transmute_copy(&langid)).into())
        }
        unsafe extern "system" fn SAPI2UPS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SAPI2UPS(this, ::core::mem::transmute_copy(&pszsapiid), ::core::mem::transmute_copy(&pszupsid), ::core::mem::transmute_copy(&cmaxlength)).into())
        }
        unsafe extern "system" fn UPS2SAPI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UPS2SAPI(this, ::core::mem::transmute_copy(&pszupsid), ::core::mem::transmute_copy(&pszsapiid), ::core::mem::transmute_copy(&cmaxlength)).into())
        }
        unsafe extern "system" fn GetMaxConvertLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL, pcmaxdestlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxConvertLength(this, ::core::mem::transmute_copy(&csrclength), ::core::mem::transmute_copy(&bsapi2ups)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcmaxdestlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpPhoneticAlphabetConverter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLangId: GetLangId::<Identity, Impl, OFFSET>,
            SetLangId: SetLangId::<Identity, Impl, OFFSET>,
            SAPI2UPS: SAPI2UPS::<Identity, Impl, OFFSET>,
            UPS2SAPI: UPS2SAPI::<Identity, Impl, OFFSET>,
            GetMaxConvertLength: GetMaxConvertLength::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpPhoneticAlphabetSelection_Impl: ::windows_core::BaseImpl {
    fn IsAlphabetUPS(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAlphabetToUPS(this: &Self::This, fforceups: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpPhoneticAlphabetSelection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpPhoneticAlphabetSelection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsAlphabetUPS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisups: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAlphabetUPS(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAlphabetToUPS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fforceups: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphabetToUPS(this, ::core::mem::transmute_copy(&fforceups)).into())
        }
        ISpPhoneticAlphabetSelection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsAlphabetUPS: IsAlphabetUPS::<Identity, Impl, OFFSET>,
            SetAlphabetToUPS: SetAlphabetToUPS::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhrase_Impl: ::windows_core::BaseImpl {
    fn GetPhrase(this: &Self::This) -> ::windows_core::Result<*mut SPPHRASE>;
    fn GetSerializedPhrase(this: &Self::This) -> ::windows_core::Result<*mut SPSERIALIZEDPHRASE>;
    fn GetText(this: &Self::This, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut ::windows_core::PWSTR, pbdisplayattributes: *mut u8) -> ::windows_core::Result<()>;
    fn Discard(this: &Self::This, dwvaluetypes: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpPhrase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpPhrase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPPHRASE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPhrase(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomemphrase, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSerializedPhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPSERIALIZEDPHRASE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSerializedPhrase(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomemphrase, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut ::windows_core::PWSTR, pbdisplayattributes: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, ::core::mem::transmute_copy(&ulstart), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&fusetextreplacements), ::core::mem::transmute_copy(&ppszcomemtext), ::core::mem::transmute_copy(&pbdisplayattributes)).into())
        }
        unsafe extern "system" fn Discard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwvaluetypes: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Discard(this, ::core::mem::transmute_copy(&dwvaluetypes)).into())
        }
        ISpPhrase_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPhrase: GetPhrase::<Identity, Impl, OFFSET>,
            GetSerializedPhrase: GetSerializedPhrase::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Discard: Discard::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhrase2_Impl: ::windows_core::BaseImpl + ISpPhrase_Impl {
    fn GetXMLResult(this: &Self::This, ppszcomemxmlresult: *mut ::windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_core::Result<()>;
    fn GetXMLErrorInfo(this: &Self::This, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_core::Result<()>;
    fn GetAudio(this: &Self::This, ulstartelement: u32, celements: u32) -> ::windows_core::Result<ISpStreamFormat>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpPhrase2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpPhrase);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhrase2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpPhrase2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetXMLResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhrase2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut ::windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetXMLResult(this, ::core::mem::transmute_copy(&ppszcomemxmlresult), ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhrase2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetXMLErrorInfo(this, ::core::mem::transmute_copy(&psemanticerrorinfo)).into())
        }
        unsafe extern "system" fn GetAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhrase2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAudio(this, ::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpPhrase2_Vtbl {
            base__: <ISpPhrase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
            GetAudio: GetAudio::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhraseAlt_Impl: ::windows_core::BaseImpl + ISpPhrase_Impl {
    fn GetAltInfo(this: &Self::This, ppparent: *mut ::core::option::Option<ISpPhrase>, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpPhraseAlt {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpPhrase);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhraseAlt_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpPhraseAlt {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAltInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhraseAlt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAltInfo(this, ::core::mem::transmute_copy(&ppparent), ::core::mem::transmute_copy(&pulstartelementinparent), ::core::mem::transmute_copy(&pcelementsinparent), ::core::mem::transmute_copy(&pcelementsinalt)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpPhraseAlt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        ISpPhraseAlt_Vtbl {
            base__: <ISpPhrase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAltInfo: GetAltInfo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpProperties_Impl: ::windows_core::BaseImpl {
    fn SetPropertyNum(this: &Self::This, pname: &::windows_core::PCWSTR, lvalue: i32) -> ::windows_core::Result<()>;
    fn GetPropertyNum(this: &Self::This, pname: &::windows_core::PCWSTR, plvalue: *mut i32) -> ::windows_core::Result<()>;
    fn SetPropertyString(this: &Self::This, pname: &::windows_core::PCWSTR, pvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPropertyString(this: &Self::This, pname: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for ISpProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPropertyNum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, lvalue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertyNum(this, ::core::mem::transmute(&pname), ::core::mem::transmute_copy(&lvalue)).into())
        }
        unsafe extern "system" fn GetPropertyNum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, plvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyNum(this, ::core::mem::transmute(&pname), ::core::mem::transmute_copy(&plvalue)).into())
        }
        unsafe extern "system" fn SetPropertyString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, pvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertyString(this, ::core::mem::transmute(&pname), ::core::mem::transmute(&pvalue)).into())
        }
        unsafe extern "system" fn GetPropertyString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, ppcomemvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyString(this, ::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomemvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPropertyNum: SetPropertyNum::<Identity, Impl, OFFSET>,
            GetPropertyNum: GetPropertyNum::<Identity, Impl, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, Impl, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
pub trait ISpRecoContext_Impl: ::windows_core::BaseImpl + ISpEventSource_Impl {
    fn GetRecognizer(this: &Self::This) -> ::windows_core::Result<ISpRecognizer>;
    fn CreateGrammar(this: &Self::This, ullgrammarid: u64) -> ::windows_core::Result<ISpRecoGrammar>;
    fn GetStatus(this: &Self::This, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows_core::Result<()>;
    fn GetMaxAlternates(this: &Self::This, pcalternates: *mut u32) -> ::windows_core::Result<()>;
    fn SetMaxAlternates(this: &Self::This, calternates: u32) -> ::windows_core::Result<()>;
    fn SetAudioOptions(this: &Self::This, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetAudioOptions(this: &Self::This, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn DeserializeResult(this: &Self::This, pserializedresult: *const SPSERIALIZEDRESULT) -> ::windows_core::Result<ISpRecoResult>;
    fn Bookmark(this: &Self::This, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetAdaptationData(this: &Self::This, padaptationdata: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This, dwreserved: u32) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This, dwreserved: u32) -> ::windows_core::Result<()>;
    fn SetVoice(this: &Self::This, pvoice: ::core::option::Option<&ISpVoice>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetVoice(this: &Self::This) -> ::windows_core::Result<ISpVoice>;
    fn SetVoicePurgeEvent(this: &Self::This, ulleventinterest: u64) -> ::windows_core::Result<()>;
    fn GetVoicePurgeEvent(this: &Self::This, pulleventinterest: *mut u64) -> ::windows_core::Result<()>;
    fn SetContextState(this: &Self::This, econtextstate: SPCONTEXTSTATE) -> ::windows_core::Result<()>;
    fn GetContextState(this: &Self::This, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl ::windows_core::Iids for ISpRecoContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpEventSource);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpRecoContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRecognizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecognizer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecognizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGrammar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullgrammarid: u64, ppgrammar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGrammar(this, ::core::mem::transmute_copy(&ullgrammarid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgrammar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pstatus)).into())
        }
        unsafe extern "system" fn GetMaxAlternates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcalternates: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaxAlternates(this, ::core::mem::transmute_copy(&pcalternates)).into())
        }
        unsafe extern "system" fn SetMaxAlternates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, calternates: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxAlternates(this, ::core::mem::transmute_copy(&calternates)).into())
        }
        unsafe extern "system" fn SetAudioOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAudioOptions(this, ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&pwaveformatex)).into())
        }
        unsafe extern "system" fn GetAudioOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAudioOptions(this, ::core::mem::transmute_copy(&poptions), ::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&ppcomemwfex)).into())
        }
        unsafe extern "system" fn DeserializeResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserializedresult: *const SPSERIALIZEDRESULT, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeserializeResult(this, ::core::mem::transmute_copy(&pserializedresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Bookmark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Bookmark(this, ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&ullstreamposition), ::core::mem::transmute_copy(&lparamevent)).into())
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padaptationdata: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAdaptationData(this, ::core::mem::transmute(&padaptationdata), ::core::mem::transmute_copy(&cch)).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this, ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this, ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn SetVoice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvoice: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVoice(this, ::windows_core::from_raw_borrowed(&pvoice), ::core::mem::transmute_copy(&fallowformatchanges)).into())
        }
        unsafe extern "system" fn GetVoice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppvoice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVoice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvoice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVoicePurgeEvent(this, ::core::mem::transmute_copy(&ulleventinterest)).into())
        }
        unsafe extern "system" fn GetVoicePurgeEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVoicePurgeEvent(this, ::core::mem::transmute_copy(&pulleventinterest)).into())
        }
        unsafe extern "system" fn SetContextState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContextState(this, ::core::mem::transmute_copy(&econtextstate)).into())
        }
        unsafe extern "system" fn GetContextState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContextState(this, ::core::mem::transmute_copy(&pecontextstate)).into())
        }
        ISpRecoContext_Vtbl {
            base__: <ISpEventSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRecognizer: GetRecognizer::<Identity, Impl, OFFSET>,
            CreateGrammar: CreateGrammar::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetMaxAlternates: GetMaxAlternates::<Identity, Impl, OFFSET>,
            SetMaxAlternates: SetMaxAlternates::<Identity, Impl, OFFSET>,
            SetAudioOptions: SetAudioOptions::<Identity, Impl, OFFSET>,
            GetAudioOptions: GetAudioOptions::<Identity, Impl, OFFSET>,
            DeserializeResult: DeserializeResult::<Identity, Impl, OFFSET>,
            Bookmark: Bookmark::<Identity, Impl, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetVoice: SetVoice::<Identity, Impl, OFFSET>,
            GetVoice: GetVoice::<Identity, Impl, OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Identity, Impl, OFFSET>,
            GetVoicePurgeEvent: GetVoicePurgeEvent::<Identity, Impl, OFFSET>,
            SetContextState: SetContextState::<Identity, Impl, OFFSET>,
            GetContextState: GetContextState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpRecoContext2_Impl: ::windows_core::BaseImpl {
    fn SetGrammarOptions(this: &Self::This, egrammaroptions: u32) -> ::windows_core::Result<()>;
    fn GetGrammarOptions(this: &Self::This, pegrammaroptions: *mut u32) -> ::windows_core::Result<()>;
    fn SetAdaptationData2(this: &Self::This, padaptationdata: &::windows_core::PCWSTR, cch: u32, ptopicname: &::windows_core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpRecoContext2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpRecoContext2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGrammarOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, egrammaroptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGrammarOptions(this, ::core::mem::transmute_copy(&egrammaroptions)).into())
        }
        unsafe extern "system" fn GetGrammarOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pegrammaroptions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGrammarOptions(this, ::core::mem::transmute_copy(&pegrammaroptions)).into())
        }
        unsafe extern "system" fn SetAdaptationData2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padaptationdata: ::windows_core::PCWSTR, cch: u32, ptopicname: ::windows_core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAdaptationData2(this, ::core::mem::transmute(&padaptationdata), ::core::mem::transmute_copy(&cch), ::core::mem::transmute(&ptopicname), ::core::mem::transmute_copy(&eadaptationsettings), ::core::mem::transmute_copy(&erelevance)).into())
        }
        ISpRecoContext2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGrammarOptions: SetGrammarOptions::<Identity, Impl, OFFSET>,
            GetGrammarOptions: GetGrammarOptions::<Identity, Impl, OFFSET>,
            SetAdaptationData2: SetAdaptationData2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpRecoGrammar_Impl: ::windows_core::BaseImpl + ISpGrammarBuilder_Impl {
    fn GetGrammarId(this: &Self::This, pullgrammarid: *mut u64) -> ::windows_core::Result<()>;
    fn GetRecoContext(this: &Self::This) -> ::windows_core::Result<ISpRecoContext>;
    fn LoadCmdFromFile(this: &Self::This, pszfilename: &::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn LoadCmdFromObject(this: &Self::This, rcid: *const ::windows_core::GUID, pszgrammarname: &::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn LoadCmdFromResource(this: &Self::This, hmodule: super::super::Foundation::HMODULE, pszresourcename: &::windows_core::PCWSTR, pszresourcetype: &::windows_core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn LoadCmdFromMemory(this: &Self::This, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn LoadCmdFromProprietaryGrammar(this: &Self::This, rguidparam: *const ::windows_core::GUID, pszstringparam: &::windows_core::PCWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn SetRuleState(this: &Self::This, pszname: &::windows_core::PCWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows_core::Result<()>;
    fn SetRuleIdState(this: &Self::This, ulruleid: u32, newstate: SPRULESTATE) -> ::windows_core::Result<()>;
    fn LoadDictation(this: &Self::This, psztopicname: &::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn UnloadDictation(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetDictationState(this: &Self::This, newstate: SPRULESTATE) -> ::windows_core::Result<()>;
    fn SetWordSequenceData(this: &Self::This, ptext: &::windows_core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_core::Result<()>;
    fn SetTextSelection(this: &Self::This, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_core::Result<()>;
    fn IsPronounceable(this: &Self::This, pszword: &::windows_core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows_core::Result<()>;
    fn SetGrammarState(this: &Self::This, egrammarstate: SPGRAMMARSTATE) -> ::windows_core::Result<()>;
    fn SaveCmd(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>, ppszcomemerrortext: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetGrammarState(this: &Self::This, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpRecoGrammar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpGrammarBuilder);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpRecoGrammar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGrammarId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pullgrammarid: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGrammarId(this, ::core::mem::transmute_copy(&pullgrammarid)).into())
        }
        unsafe extern "system" fn GetRecoContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprecoctxt: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecoContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecoctxt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadCmdFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadCmdFromFile(this, ::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn LoadCmdFromObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rcid: *const ::windows_core::GUID, pszgrammarname: ::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadCmdFromObject(this, ::core::mem::transmute_copy(&rcid), ::core::mem::transmute(&pszgrammarname), ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn LoadCmdFromResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HMODULE, pszresourcename: ::windows_core::PCWSTR, pszresourcetype: ::windows_core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadCmdFromResource(this, ::core::mem::transmute_copy(&hmodule), ::core::mem::transmute(&pszresourcename), ::core::mem::transmute(&pszresourcetype), ::core::mem::transmute_copy(&wlanguage), ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn LoadCmdFromMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadCmdFromMemory(this, ::core::mem::transmute_copy(&pgrammar), ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn LoadCmdFromProprietaryGrammar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidparam: *const ::windows_core::GUID, pszstringparam: ::windows_core::PCWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadCmdFromProprietaryGrammar(this, ::core::mem::transmute_copy(&rguidparam), ::core::mem::transmute(&pszstringparam), ::core::mem::transmute_copy(&pvdataprarm), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn SetRuleState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRuleState(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&preserved), ::core::mem::transmute_copy(&newstate)).into())
        }
        unsafe extern "system" fn SetRuleIdState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulruleid: u32, newstate: SPRULESTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRuleIdState(this, ::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&newstate)).into())
        }
        unsafe extern "system" fn LoadDictation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztopicname: ::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadDictation(this, ::core::mem::transmute(&psztopicname), ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn UnloadDictation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnloadDictation(this).into())
        }
        unsafe extern "system" fn SetDictationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDictationState(this, ::core::mem::transmute_copy(&newstate)).into())
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: ::windows_core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWordSequenceData(this, ::core::mem::transmute(&ptext), ::core::mem::transmute_copy(&cchtext), ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn SetTextSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextSelection(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn IsPronounceable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPronounceable(this, ::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&pwordpronounceable)).into())
        }
        unsafe extern "system" fn SetGrammarState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGrammarState(this, ::core::mem::transmute_copy(&egrammarstate)).into())
        }
        unsafe extern "system" fn SaveCmd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ppszcomemerrortext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveCmd(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&ppszcomemerrortext)).into())
        }
        unsafe extern "system" fn GetGrammarState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGrammarState(this, ::core::mem::transmute_copy(&pegrammarstate)).into())
        }
        ISpRecoGrammar_Vtbl {
            base__: <ISpGrammarBuilder as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGrammarId: GetGrammarId::<Identity, Impl, OFFSET>,
            GetRecoContext: GetRecoContext::<Identity, Impl, OFFSET>,
            LoadCmdFromFile: LoadCmdFromFile::<Identity, Impl, OFFSET>,
            LoadCmdFromObject: LoadCmdFromObject::<Identity, Impl, OFFSET>,
            LoadCmdFromResource: LoadCmdFromResource::<Identity, Impl, OFFSET>,
            LoadCmdFromMemory: LoadCmdFromMemory::<Identity, Impl, OFFSET>,
            LoadCmdFromProprietaryGrammar: LoadCmdFromProprietaryGrammar::<Identity, Impl, OFFSET>,
            SetRuleState: SetRuleState::<Identity, Impl, OFFSET>,
            SetRuleIdState: SetRuleIdState::<Identity, Impl, OFFSET>,
            LoadDictation: LoadDictation::<Identity, Impl, OFFSET>,
            UnloadDictation: UnloadDictation::<Identity, Impl, OFFSET>,
            SetDictationState: SetDictationState::<Identity, Impl, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, Impl, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, Impl, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, Impl, OFFSET>,
            SetGrammarState: SetGrammarState::<Identity, Impl, OFFSET>,
            SaveCmd: SaveCmd::<Identity, Impl, OFFSET>,
            GetGrammarState: GetGrammarState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_Urlmon\"`"]
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub trait ISpRecoGrammar2_Impl: ::windows_core::BaseImpl {
    fn GetRules(this: &Self::This, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows_core::Result<()>;
    fn LoadCmdFromFile2(this: &Self::This, pszfilename: &::windows_core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: &::windows_core::PCWSTR, pszbaseuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn LoadCmdFromMemory2(this: &Self::This, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: &::windows_core::PCWSTR, pszbaseuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetRulePriority(this: &Self::This, pszrulename: &::windows_core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows_core::Result<()>;
    fn SetRuleWeight(this: &Self::This, pszrulename: &::windows_core::PCWSTR, ulruleid: u32, flweight: f32) -> ::windows_core::Result<()>;
    fn SetDictationWeight(this: &Self::This, flweight: f32) -> ::windows_core::Result<()>;
    fn SetGrammarLoader(this: &Self::This, ploader: ::core::option::Option<&ISpeechResourceLoader>) -> ::windows_core::Result<()>;
    fn SetSMLSecurityManager(this: &Self::This, psmlsecuritymanager: ::core::option::Option<&super::super::System::Com::Urlmon::IInternetSecurityManager>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_Urlmon")]
impl ::windows_core::Iids for ISpRecoGrammar2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_Urlmon")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpRecoGrammar2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRules(this, ::core::mem::transmute_copy(&ppcomemrules), ::core::mem::transmute_copy(&punumrules)).into())
        }
        unsafe extern "system" fn LoadCmdFromFile2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: ::windows_core::PCWSTR, pszbaseuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadCmdFromFile2(this, ::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&pszsharinguri), ::core::mem::transmute(&pszbaseuri)).into())
        }
        unsafe extern "system" fn LoadCmdFromMemory2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: ::windows_core::PCWSTR, pszbaseuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadCmdFromMemory2(this, ::core::mem::transmute_copy(&pgrammar), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&pszsharinguri), ::core::mem::transmute(&pszbaseuri)).into())
        }
        unsafe extern "system" fn SetRulePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrulename: ::windows_core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRulePriority(this, ::core::mem::transmute(&pszrulename), ::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&nrulepriority)).into())
        }
        unsafe extern "system" fn SetRuleWeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrulename: ::windows_core::PCWSTR, ulruleid: u32, flweight: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRuleWeight(this, ::core::mem::transmute(&pszrulename), ::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&flweight)).into())
        }
        unsafe extern "system" fn SetDictationWeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flweight: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDictationWeight(this, ::core::mem::transmute_copy(&flweight)).into())
        }
        unsafe extern "system" fn SetGrammarLoader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGrammarLoader(this, ::windows_core::from_raw_borrowed(&ploader)).into())
        }
        unsafe extern "system" fn SetSMLSecurityManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psmlsecuritymanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSMLSecurityManager(this, ::windows_core::from_raw_borrowed(&psmlsecuritymanager)).into())
        }
        ISpRecoGrammar2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRules: GetRules::<Identity, Impl, OFFSET>,
            LoadCmdFromFile2: LoadCmdFromFile2::<Identity, Impl, OFFSET>,
            LoadCmdFromMemory2: LoadCmdFromMemory2::<Identity, Impl, OFFSET>,
            SetRulePriority: SetRulePriority::<Identity, Impl, OFFSET>,
            SetRuleWeight: SetRuleWeight::<Identity, Impl, OFFSET>,
            SetDictationWeight: SetDictationWeight::<Identity, Impl, OFFSET>,
            SetGrammarLoader: SetGrammarLoader::<Identity, Impl, OFFSET>,
            SetSMLSecurityManager: SetSMLSecurityManager::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpRecoResult_Impl: ::windows_core::BaseImpl + ISpPhrase_Impl {
    fn GetResultTimes(this: &Self::This, ptimes: *mut SPRECORESULTTIMES) -> ::windows_core::Result<()>;
    fn GetAlternates(this: &Self::This, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut ::core::option::Option<ISpPhraseAlt>, pcphrasesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn GetAudio(this: &Self::This, ulstartelement: u32, celements: u32) -> ::windows_core::Result<ISpStreamFormat>;
    fn SpeakAudio(this: &Self::This, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::Result<()>;
    fn Serialize(this: &Self::This, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows_core::Result<()>;
    fn ScaleAudio(this: &Self::This, paudioformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetRecoContext(this: &Self::This) -> ::windows_core::Result<ISpRecoContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpRecoResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpPhrase);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpRecoResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResultTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimes: *mut SPRECORESULTTIMES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResultTimes(this, ::core::mem::transmute_copy(&ptimes)).into())
        }
        unsafe extern "system" fn GetAlternates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut *mut ::core::ffi::c_void, pcphrasesreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAlternates(this, ::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&ulrequestcount), ::core::mem::transmute_copy(&ppphrases), ::core::mem::transmute_copy(&pcphrasesreturned)).into())
        }
        unsafe extern "system" fn GetAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAudio(this, ::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpeakAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SpeakAudio(this, ::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pulstreamnumber)).into())
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&ppcomemserializedresult)).into())
        }
        unsafe extern "system" fn ScaleAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paudioformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScaleAudio(this, ::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&pwaveformatex)).into())
        }
        unsafe extern "system" fn GetRecoContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprecocontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecoContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecocontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpRecoResult_Vtbl {
            base__: <ISpPhrase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetResultTimes: GetResultTimes::<Identity, Impl, OFFSET>,
            GetAlternates: GetAlternates::<Identity, Impl, OFFSET>,
            GetAudio: GetAudio::<Identity, Impl, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            ScaleAudio: ScaleAudio::<Identity, Impl, OFFSET>,
            GetRecoContext: GetRecoContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpRecoResult2_Impl: ::windows_core::BaseImpl + ISpRecoResult_Impl {
    fn CommitAlternate(this: &Self::This, pphrasealt: ::core::option::Option<&ISpPhraseAlt>) -> ::windows_core::Result<ISpRecoResult>;
    fn CommitText(this: &Self::This, ulstartelement: u32, celements: u32, pszcorrecteddata: &::windows_core::PCWSTR, ecommitflags: u32) -> ::windows_core::Result<()>;
    fn SetTextFeedback(this: &Self::This, pszfeedback: &::windows_core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpRecoResult2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpRecoResult);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpRecoResult2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommitAlternate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphrasealt: *mut ::core::ffi::c_void, ppnewresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitAlternate(this, ::windows_core::from_raw_borrowed(&pphrasealt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommitText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, pszcorrecteddata: ::windows_core::PCWSTR, ecommitflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitText(this, ::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute(&pszcorrecteddata), ::core::mem::transmute_copy(&ecommitflags)).into())
        }
        unsafe extern "system" fn SetTextFeedback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecoResult2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfeedback: ::windows_core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextFeedback(this, ::core::mem::transmute(&pszfeedback), ::core::mem::transmute_copy(&fsuccessful)).into())
        }
        ISpRecoResult2_Vtbl {
            base__: <ISpRecoResult as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommitAlternate: CommitAlternate::<Identity, Impl, OFFSET>,
            CommitText: CommitText::<Identity, Impl, OFFSET>,
            SetTextFeedback: SetTextFeedback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpRecognizer_Impl: ::windows_core::BaseImpl + ISpProperties_Impl {
    fn SetRecognizer(this: &Self::This, precognizer: ::core::option::Option<&ISpObjectToken>) -> ::windows_core::Result<()>;
    fn GetRecognizer(this: &Self::This) -> ::windows_core::Result<ISpObjectToken>;
    fn SetInput(this: &Self::This, punkinput: ::core::option::Option<&::windows_core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetInputObjectToken(this: &Self::This) -> ::windows_core::Result<ISpObjectToken>;
    fn GetInputStream(this: &Self::This) -> ::windows_core::Result<ISpStreamFormat>;
    fn CreateRecoContext(this: &Self::This) -> ::windows_core::Result<ISpRecoContext>;
    fn GetRecoProfile(this: &Self::This) -> ::windows_core::Result<ISpObjectToken>;
    fn SetRecoProfile(this: &Self::This, ptoken: ::core::option::Option<&ISpObjectToken>) -> ::windows_core::Result<()>;
    fn IsSharedInstance(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetRecoState(this: &Self::This, pstate: *mut SPRECOSTATE) -> ::windows_core::Result<()>;
    fn SetRecoState(this: &Self::This, newstate: SPRECOSTATE) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows_core::Result<()>;
    fn GetFormat(this: &Self::This, waveformattype: SPSTREAMFORMATTYPE, pformatid: *mut ::windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn IsUISupported(this: &Self::This, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DisplayUI(this: &Self::This, hwndparent: super::super::Foundation::HWND, psztitle: &::windows_core::PCWSTR, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_core::Result<()>;
    fn EmulateRecognition(this: &Self::This, pphrase: ::core::option::Option<&ISpPhrase>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpRecognizer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpProperties);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpRecognizer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRecognizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precognizer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecognizer(this, ::windows_core::from_raw_borrowed(&precognizer)).into())
        }
        unsafe extern "system" fn GetRecognizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecognizer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecognizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkinput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInput(this, ::windows_core::from_raw_borrowed(&punkinput), ::core::mem::transmute_copy(&fallowformatchanges)).into())
        }
        unsafe extern "system" fn GetInputObjectToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputObjectToken(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRecoContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnewctxt: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRecoContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewctxt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRecoProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecoProfile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRecoProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecoProfile(this, ::windows_core::from_raw_borrowed(&ptoken)).into())
        }
        unsafe extern "system" fn IsSharedInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSharedInstance(this).into())
        }
        unsafe extern "system" fn GetRecoState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut SPRECOSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRecoState(this, ::core::mem::transmute_copy(&pstate)).into())
        }
        unsafe extern "system" fn SetRecoState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstate: SPRECOSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecoState(this, ::core::mem::transmute_copy(&newstate)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pstatus)).into())
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waveformattype: SPSTREAMFORMATTYPE, pformatid: *mut ::windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFormat(this, ::core::mem::transmute_copy(&waveformattype), ::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&ppcomemwfex)).into())
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUISupported(this, ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute_copy(&pfsupported)).into())
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows_core::PCWSTR, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayUI(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&psztitle), ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata)).into())
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphrase: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EmulateRecognition(this, ::windows_core::from_raw_borrowed(&pphrase)).into())
        }
        ISpRecognizer_Vtbl {
            base__: <ISpProperties as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRecognizer: SetRecognizer::<Identity, Impl, OFFSET>,
            GetRecognizer: GetRecognizer::<Identity, Impl, OFFSET>,
            SetInput: SetInput::<Identity, Impl, OFFSET>,
            GetInputObjectToken: GetInputObjectToken::<Identity, Impl, OFFSET>,
            GetInputStream: GetInputStream::<Identity, Impl, OFFSET>,
            CreateRecoContext: CreateRecoContext::<Identity, Impl, OFFSET>,
            GetRecoProfile: GetRecoProfile::<Identity, Impl, OFFSET>,
            SetRecoProfile: SetRecoProfile::<Identity, Impl, OFFSET>,
            IsSharedInstance: IsSharedInstance::<Identity, Impl, OFFSET>,
            GetRecoState: GetRecoState::<Identity, Impl, OFFSET>,
            SetRecoState: SetRecoState::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpRecognizer2_Impl: ::windows_core::BaseImpl {
    fn EmulateRecognitionEx(this: &Self::This, pphrase: ::core::option::Option<&ISpPhrase>, dwcompareflags: u32) -> ::windows_core::Result<()>;
    fn SetTrainingState(this: &Self::This, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ResetAcousticModelAdaptation(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpRecognizer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpRecognizer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EmulateRecognitionEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphrase: *mut ::core::ffi::c_void, dwcompareflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EmulateRecognitionEx(this, ::windows_core::from_raw_borrowed(&pphrase), ::core::mem::transmute_copy(&dwcompareflags)).into())
        }
        unsafe extern "system" fn SetTrainingState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrainingState(this, ::core::mem::transmute_copy(&fdoingtraining), ::core::mem::transmute_copy(&fadaptfromtrainingdata)).into())
        }
        unsafe extern "system" fn ResetAcousticModelAdaptation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRecognizer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetAcousticModelAdaptation(this).into())
        }
        ISpRecognizer2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EmulateRecognitionEx: EmulateRecognitionEx::<Identity, Impl, OFFSET>,
            SetTrainingState: SetTrainingState::<Identity, Impl, OFFSET>,
            ResetAcousticModelAdaptation: ResetAcousticModelAdaptation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait ISpRegDataKey_Impl: ::windows_core::BaseImpl + ISpDataKey_Impl {
    fn SetKey(this: &Self::This, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::Iids for ISpRegDataKey {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpDataKey);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRegDataKey_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpRegDataKey {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpRegDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKey(this, ::core::mem::transmute_copy(&hkey), ::core::mem::transmute_copy(&freadonly)).into())
        }
        ISpRegDataKey_Vtbl { base__: <ISpDataKey as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetKey: SetKey::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpResourceManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IServiceProvider_Impl {
    fn SetObject(this: &Self::This, guidserviceid: *const ::windows_core::GUID, punkobject: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetObject(this: &Self::This, guidserviceid: *const ::windows_core::GUID, objectclsid: *const ::windows_core::GUID, objectiid: *const ::windows_core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpResourceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IServiceProvider);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpResourceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpResourceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpResourceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows_core::GUID, punkobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObject(this, ::core::mem::transmute_copy(&guidserviceid), ::windows_core::from_raw_borrowed(&punkobject)).into())
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpResourceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows_core::GUID, objectclsid: *const ::windows_core::GUID, objectiid: *const ::windows_core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObject(this, ::core::mem::transmute_copy(&guidserviceid), ::core::mem::transmute_copy(&objectclsid), ::core::mem::transmute_copy(&objectiid), ::core::mem::transmute_copy(&freleasewhenlastexternalrefreleased), ::core::mem::transmute_copy(&ppobject)).into())
        }
        ISpResourceManager_Vtbl {
            base__: <super::super::System::Com::IServiceProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetObject: SetObject::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpSerializeState_Impl: ::windows_core::BaseImpl {
    fn GetSerializedState(this: &Self::This, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn SetSerializedState(this: &Self::This, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpSerializeState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpSerializeState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpSerializeState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSerializedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpSerializeState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSerializedState(this, ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pulsize), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn SetSerializedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpSerializeState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSerializedState(this, ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        ISpSerializeState_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSerializedState: GetSerializedState::<Identity, Impl, OFFSET>,
            SetSerializedState: SetSerializedState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpShortcut_Impl: ::windows_core::BaseImpl {
    fn AddShortcut(this: &Self::This, pszdisplay: &::windows_core::PCWSTR, langid: u16, pszspoken: &::windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_core::Result<()>;
    fn RemoveShortcut(this: &Self::This, pszdisplay: &::windows_core::PCWSTR, langid: u16, pszspoken: &::windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_core::Result<()>;
    fn GetShortcuts(this: &Self::This, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::Result<()>;
    fn GetGeneration(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetWordsFromGenerationChange(this: &Self::This, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::Result<()>;
    fn GetWords(this: &Self::This, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::Result<()>;
    fn GetShortcutsForGeneration(this: &Self::This, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::Result<()>;
    fn GetGenerationChange(this: &Self::This, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpShortcut {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpShortcut {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddShortcut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdisplay: ::windows_core::PCWSTR, langid: u16, pszspoken: ::windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddShortcut(this, ::core::mem::transmute(&pszdisplay), ::core::mem::transmute_copy(&langid), ::core::mem::transmute(&pszspoken), ::core::mem::transmute_copy(&shtype)).into())
        }
        unsafe extern "system" fn RemoveShortcut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdisplay: ::windows_core::PCWSTR, langid: u16, pszspoken: ::windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveShortcut(this, ::core::mem::transmute(&pszdisplay), ::core::mem::transmute_copy(&langid), ::core::mem::transmute(&pszspoken), ::core::mem::transmute_copy(&shtype)).into())
        }
        unsafe extern "system" fn GetShortcuts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetShortcuts(this, ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pshortcutpairlist)).into())
        }
        unsafe extern "system" fn GetGeneration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGeneration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwgeneration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWordsFromGenerationChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWordsFromGenerationChange(this, ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pwordlist)).into())
        }
        unsafe extern "system" fn GetWords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWords(this, ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pwordlist)).into())
        }
        unsafe extern "system" fn GetShortcutsForGeneration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetShortcutsForGeneration(this, ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pshortcutpairlist)).into())
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGenerationChange(this, ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pshortcutpairlist)).into())
        }
        ISpShortcut_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddShortcut: AddShortcut::<Identity, Impl, OFFSET>,
            RemoveShortcut: RemoveShortcut::<Identity, Impl, OFFSET>,
            GetShortcuts: GetShortcuts::<Identity, Impl, OFFSET>,
            GetGeneration: GetGeneration::<Identity, Impl, OFFSET>,
            GetWordsFromGenerationChange: GetWordsFromGenerationChange::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
            GetShortcutsForGeneration: GetShortcutsForGeneration::<Identity, Impl, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpStream_Impl: ::windows_core::BaseImpl + ISpStreamFormat_Impl {
    fn SetBaseStream(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>, rguidformat: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetBaseStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn BindToFile(this: &Self::This, pszfilename: &::windows_core::PCWSTR, emode: SPFILEMODE, pformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpStreamFormat);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBaseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, rguidformat: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBaseStream(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&rguidformat), ::core::mem::transmute_copy(&pwaveformatex)).into())
        }
        unsafe extern "system" fn GetBaseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBaseStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BindToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, emode: SPFILEMODE, pformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindToFile(this, ::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&emode), ::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&ulleventinterest)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        ISpStream_Vtbl {
            base__: <ISpStreamFormat as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBaseStream: SetBaseStream::<Identity, Impl, OFFSET>,
            GetBaseStream: GetBaseStream::<Identity, Impl, OFFSET>,
            BindToFile: BindToFile::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpStreamFormat_Impl: ::windows_core::BaseImpl + super::super::System::Com::IStream_Impl {
    fn GetFormat(this: &Self::This, pguidformatid: *const ::windows_core::GUID) -> ::windows_core::Result<*mut super::Audio::WAVEFORMATEX>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpStreamFormat {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStreamFormat_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpStreamFormat {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStreamFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidformatid: *const ::windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormat(this, ::core::mem::transmute_copy(&pguidformatid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomemwaveformatex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpStreamFormat_Vtbl { base__: <super::super::System::Com::IStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetFormat: GetFormat::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpStreamFormatConverter_Impl: ::windows_core::BaseImpl + ISpStreamFormat_Impl {
    fn SetBaseStream(this: &Self::This, pstream: ::core::option::Option<&ISpStreamFormat>, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetBaseStream(this: &Self::This) -> ::windows_core::Result<ISpStreamFormat>;
    fn SetFormat(this: &Self::This, rguidformatidofconvertedstream: *const ::windows_core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn ResetSeekPosition(this: &Self::This) -> ::windows_core::Result<()>;
    fn ScaleConvertedToBaseOffset(this: &Self::This, ulloffsetconvertedstream: u64) -> ::windows_core::Result<u64>;
    fn ScaleBaseToConvertedOffset(this: &Self::This, ulloffsetbasestream: u64) -> ::windows_core::Result<u64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpStreamFormatConverter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpStreamFormat);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpStreamFormatConverter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBaseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBaseStream(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&fsetformattobasestreamformat), ::core::mem::transmute_copy(&fwritetobasestream)).into())
        }
        unsafe extern "system" fn GetBaseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBaseStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidformatidofconvertedstream: *const ::windows_core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormat(this, ::core::mem::transmute_copy(&rguidformatidofconvertedstream), ::core::mem::transmute_copy(&pwaveformatexofconvertedstream)).into())
        }
        unsafe extern "system" fn ResetSeekPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetSeekPosition(this).into())
        }
        unsafe extern "system" fn ScaleConvertedToBaseOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffsetconvertedstream: u64, pulloffsetbasestream: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScaleConvertedToBaseOffset(this, ::core::mem::transmute_copy(&ulloffsetconvertedstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulloffsetbasestream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScaleBaseToConvertedOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulloffsetbasestream: u64, pulloffsetconvertedstream: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScaleBaseToConvertedOffset(this, ::core::mem::transmute_copy(&ulloffsetbasestream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulloffsetconvertedstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpStreamFormatConverter_Vtbl {
            base__: <ISpStreamFormat as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBaseStream: SetBaseStream::<Identity, Impl, OFFSET>,
            GetBaseStream: GetBaseStream::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            ResetSeekPosition: ResetSeekPosition::<Identity, Impl, OFFSET>,
            ScaleConvertedToBaseOffset: ScaleConvertedToBaseOffset::<Identity, Impl, OFFSET>,
            ScaleBaseToConvertedOffset: ScaleBaseToConvertedOffset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpTranscript_Impl: ::windows_core::BaseImpl {
    fn GetTranscript(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn AppendTranscript(this: &Self::This, psztranscript: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpTranscript {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpTranscript_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpTranscript {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTranscript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpTranscript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsztranscript: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTranscript(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztranscript, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppendTranscript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpTranscript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztranscript: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppendTranscript(this, ::core::mem::transmute(&psztranscript)).into())
        }
        ISpTranscript_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTranscript: GetTranscript::<Identity, Impl, OFFSET>,
            AppendTranscript: AppendTranscript::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpVoice_Impl: ::windows_core::BaseImpl + ISpEventSource_Impl {
    fn SetOutput(this: &Self::This, punkoutput: ::core::option::Option<&::windows_core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetOutputObjectToken(this: &Self::This) -> ::windows_core::Result<ISpObjectToken>;
    fn GetOutputStream(this: &Self::This) -> ::windows_core::Result<ISpStreamFormat>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetVoice(this: &Self::This, ptoken: ::core::option::Option<&ISpObjectToken>) -> ::windows_core::Result<()>;
    fn GetVoice(this: &Self::This) -> ::windows_core::Result<ISpObjectToken>;
    fn Speak(this: &Self::This, pwcs: &::windows_core::PCWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::Result<()>;
    fn SpeakStream(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, pitemtype: &::windows_core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows_core::Result<()>;
    fn SetPriority(this: &Self::This, epriority: SPVPRIORITY) -> ::windows_core::Result<()>;
    fn GetPriority(this: &Self::This, pepriority: *mut SPVPRIORITY) -> ::windows_core::Result<()>;
    fn SetAlertBoundary(this: &Self::This, eboundary: SPEVENTENUM) -> ::windows_core::Result<()>;
    fn GetAlertBoundary(this: &Self::This, peboundary: *mut SPEVENTENUM) -> ::windows_core::Result<()>;
    fn SetRate(this: &Self::This, rateadjust: i32) -> ::windows_core::Result<()>;
    fn GetRate(this: &Self::This, prateadjust: *mut i32) -> ::windows_core::Result<()>;
    fn SetVolume(this: &Self::This, usvolume: u16) -> ::windows_core::Result<()>;
    fn GetVolume(this: &Self::This, pusvolume: *mut u16) -> ::windows_core::Result<()>;
    fn WaitUntilDone(this: &Self::This, mstimeout: u32) -> ::windows_core::Result<()>;
    fn SetSyncSpeakTimeout(this: &Self::This, mstimeout: u32) -> ::windows_core::Result<()>;
    fn GetSyncSpeakTimeout(this: &Self::This, pmstimeout: *mut u32) -> ::windows_core::Result<()>;
    fn SpeakCompleteEvent(this: &Self::This) -> super::super::Foundation::HANDLE;
    fn IsUISupported(this: &Self::This, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DisplayUI(this: &Self::This, hwndparent: super::super::Foundation::HWND, psztitle: &::windows_core::PCWSTR, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpVoice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpEventSource);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpVoice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkoutput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutput(this, ::windows_core::from_raw_borrowed(&punkoutput), ::core::mem::transmute_copy(&fallowformatchanges)).into())
        }
        unsafe extern "system" fn GetOutputObjectToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppobjecttoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputObjectToken(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjecttoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn SetVoice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVoice(this, ::windows_core::from_raw_borrowed(&ptoken)).into())
        }
        unsafe extern "system" fn GetVoice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVoice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Speak<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcs: ::windows_core::PCWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Speak(this, ::core::mem::transmute(&pwcs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pulstreamnumber)).into())
        }
        unsafe extern "system" fn SpeakStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SpeakStream(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pulstreamnumber)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppszlastbookmark)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemtype: ::windows_core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute(&pitemtype), ::core::mem::transmute_copy(&lnumitems), ::core::mem::transmute_copy(&pulnumskipped)).into())
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, epriority: SPVPRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&epriority)).into())
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pepriority: *mut SPVPRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPriority(this, ::core::mem::transmute_copy(&pepriority)).into())
        }
        unsafe extern "system" fn SetAlertBoundary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eboundary: SPEVENTENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlertBoundary(this, ::core::mem::transmute_copy(&eboundary)).into())
        }
        unsafe extern "system" fn GetAlertBoundary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peboundary: *mut SPEVENTENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAlertBoundary(this, ::core::mem::transmute_copy(&peboundary)).into())
        }
        unsafe extern "system" fn SetRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rateadjust: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRate(this, ::core::mem::transmute_copy(&rateadjust)).into())
        }
        unsafe extern "system" fn GetRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prateadjust: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRate(this, ::core::mem::transmute_copy(&prateadjust)).into())
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usvolume: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolume(this, ::core::mem::transmute_copy(&usvolume)).into())
        }
        unsafe extern "system" fn GetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusvolume: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVolume(this, ::core::mem::transmute_copy(&pusvolume)).into())
        }
        unsafe extern "system" fn WaitUntilDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitUntilDone(this, ::core::mem::transmute_copy(&mstimeout)).into())
        }
        unsafe extern "system" fn SetSyncSpeakTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSyncSpeakTimeout(this, ::core::mem::transmute_copy(&mstimeout)).into())
        }
        unsafe extern "system" fn GetSyncSpeakTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmstimeout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSyncSpeakTimeout(this, ::core::mem::transmute_copy(&pmstimeout)).into())
        }
        unsafe extern "system" fn SpeakCompleteEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SpeakCompleteEvent(this))
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUISupported(this, ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute_copy(&pfsupported)).into())
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows_core::PCWSTR, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayUI(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&psztitle), ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata)).into())
        }
        ISpVoice_Vtbl {
            base__: <ISpEventSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOutput: SetOutput::<Identity, Impl, OFFSET>,
            GetOutputObjectToken: GetOutputObjectToken::<Identity, Impl, OFFSET>,
            GetOutputStream: GetOutputStream::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetVoice: SetVoice::<Identity, Impl, OFFSET>,
            GetVoice: GetVoice::<Identity, Impl, OFFSET>,
            Speak: Speak::<Identity, Impl, OFFSET>,
            SpeakStream: SpeakStream::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Identity, Impl, OFFSET>,
            GetAlertBoundary: GetAlertBoundary::<Identity, Impl, OFFSET>,
            SetRate: SetRate::<Identity, Impl, OFFSET>,
            GetRate: GetRate::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            GetVolume: GetVolume::<Identity, Impl, OFFSET>,
            WaitUntilDone: WaitUntilDone::<Identity, Impl, OFFSET>,
            SetSyncSpeakTimeout: SetSyncSpeakTimeout::<Identity, Impl, OFFSET>,
            GetSyncSpeakTimeout: GetSyncSpeakTimeout::<Identity, Impl, OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpXMLRecoResult_Impl: ::windows_core::BaseImpl + ISpRecoResult_Impl {
    fn GetXMLResult(this: &Self::This, ppszcomemxmlresult: *mut ::windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_core::Result<()>;
    fn GetXMLErrorInfo(this: &Self::This, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpXMLRecoResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpRecoResult);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpXMLRecoResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpXMLRecoResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetXMLResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpXMLRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut ::windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetXMLResult(this, ::core::mem::transmute_copy(&ppszcomemxmlresult), ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpXMLRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetXMLErrorInfo(this, ::core::mem::transmute_copy(&psemanticerrorinfo)).into())
        }
        ISpXMLRecoResult_Vtbl {
            base__: <ISpRecoResult as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudio_Impl: ::windows_core::BaseImpl + ISpeechBaseStream_Impl {
    fn Status(this: &Self::This) -> ::windows_core::Result<ISpeechAudioStatus>;
    fn BufferInfo(this: &Self::This) -> ::windows_core::Result<ISpeechAudioBufferInfo>;
    fn DefaultFormat(this: &Self::This) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn Volume(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetVolume(this: &Self::This, volume: i32) -> ::windows_core::Result<()>;
    fn BufferNotifySize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBufferNotifySize(this: &Self::This, buffernotifysize: i32) -> ::windows_core::Result<()>;
    fn EventHandle(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetState(this: &Self::This, state: SpeechAudioState) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechAudio {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpeechBaseStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechAudio {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BufferInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bufferinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DefaultFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Volume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Volume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(volume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolume(this, ::core::mem::transmute_copy(&volume)).into())
        }
        unsafe extern "system" fn BufferNotifySize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffernotifysize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferNotifySize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffernotifysize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffernotifysize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferNotifySize(this, ::core::mem::transmute_copy(&buffernotifysize)).into())
        }
        unsafe extern "system" fn EventHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandle: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: SpeechAudioState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&state)).into())
        }
        ISpeechAudio_Vtbl {
            base__: <ISpeechBaseStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            BufferInfo: BufferInfo::<Identity, Impl, OFFSET>,
            DefaultFormat: DefaultFormat::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            BufferNotifySize: BufferNotifySize::<Identity, Impl, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, Impl, OFFSET>,
            EventHandle: EventHandle::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudioBufferInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn MinNotification(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMinNotification(this: &Self::This, minnotification: i32) -> ::windows_core::Result<()>;
    fn BufferSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBufferSize(this: &Self::This, buffersize: i32) -> ::windows_core::Result<()>;
    fn EventBias(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEventBias(this: &Self::This, eventbias: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechAudioBufferInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechAudioBufferInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MinNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minnotification: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinNotification(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minnotification, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minnotification: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinNotification(this, ::core::mem::transmute_copy(&minnotification)).into())
        }
        unsafe extern "system" fn BufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffersize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffersize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffersize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferSize(this, ::core::mem::transmute_copy(&buffersize)).into())
        }
        unsafe extern "system" fn EventBias<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventbias: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventBias(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventbias, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventBias<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventbias: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventBias(this, ::core::mem::transmute_copy(&eventbias)).into())
        }
        ISpeechAudioBufferInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MinNotification: MinNotification::<Identity, Impl, OFFSET>,
            SetMinNotification: SetMinNotification::<Identity, Impl, OFFSET>,
            BufferSize: BufferSize::<Identity, Impl, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, Impl, OFFSET>,
            EventBias: EventBias::<Identity, Impl, OFFSET>,
            SetEventBias: SetEventBias::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudioFormat_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<SpeechAudioFormatType>;
    fn SetType(this: &Self::This, audioformat: SpeechAudioFormatType) -> ::windows_core::Result<()>;
    fn Guid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetGuid(this: &Self::This, guid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetWaveFormatEx(this: &Self::This) -> ::windows_core::Result<ISpeechWaveFormatEx>;
    fn SetWaveFormatEx(this: &Self::This, speechwaveformatex: ::core::option::Option<&ISpeechWaveFormatEx>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechAudioFormat {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechAudioFormat {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audioformat: *mut SpeechAudioFormatType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audioformat: SpeechAudioFormatType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&audioformat)).into())
        }
        unsafe extern "system" fn Guid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Guid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(guid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGuid(this, ::core::mem::transmute(&guid)).into())
        }
        unsafe extern "system" fn GetWaveFormatEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speechwaveformatex: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWaveFormatEx(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(speechwaveformatex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWaveFormatEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speechwaveformatex: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWaveFormatEx(this, ::windows_core::from_raw_borrowed(&speechwaveformatex)).into())
        }
        ISpeechAudioFormat_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Guid: Guid::<Identity, Impl, OFFSET>,
            SetGuid: SetGuid::<Identity, Impl, OFFSET>,
            GetWaveFormatEx: GetWaveFormatEx::<Identity, Impl, OFFSET>,
            SetWaveFormatEx: SetWaveFormatEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudioStatus_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn FreeBufferSpace(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NonBlockingIO(this: &Self::This) -> ::windows_core::Result<i32>;
    fn State(this: &Self::This) -> ::windows_core::Result<SpeechAudioState>;
    fn CurrentSeekPosition(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CurrentDevicePosition(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechAudioStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechAudioStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FreeBufferSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, freebufferspace: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FreeBufferSpace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(freebufferspace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NonBlockingIO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nonblockingio: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NonBlockingIO(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nonblockingio, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut SpeechAudioState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentSeekPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentseekposition: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentSeekPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentseekposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentDevicePosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentdeviceposition: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDevicePosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentdeviceposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechAudioStatus_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FreeBufferSpace: FreeBufferSpace::<Identity, Impl, OFFSET>,
            NonBlockingIO: NonBlockingIO::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            CurrentSeekPosition: CurrentSeekPosition::<Identity, Impl, OFFSET>,
            CurrentDevicePosition: CurrentDevicePosition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechBaseStream_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Format(this: &Self::This) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn putref_Format(this: &Self::This, audioformat: ::core::option::Option<&ISpeechAudioFormat>) -> ::windows_core::Result<()>;
    fn Read(this: &Self::This, buffer: *mut super::super::System::Variant::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows_core::Result<()>;
    fn Write(this: &Self::This, buffer: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<i32>;
    fn Seek(this: &Self::This, position: &super::super::System::Variant::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechBaseStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechBaseStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Format<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audioformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Format(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_Format<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audioformat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_Format(this, ::windows_core::from_raw_borrowed(&audioformat)).into())
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *mut super::super::System::Variant::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&numberofbytes), ::core::mem::transmute_copy(&bytesread)).into())
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: super::super::System::Variant::VARIANT, byteswritten: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Write(this, ::core::mem::transmute(&buffer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(byteswritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: super::super::System::Variant::VARIANT, origin: SpeechStreamSeekPositionType, newposition: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Seek(this, ::core::mem::transmute(&position), ::core::mem::transmute_copy(&origin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechBaseStream_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Format: Format::<Identity, Impl, OFFSET>,
            putref_Format: putref_Format::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechCustomStream_Impl: ::windows_core::BaseImpl + ISpeechBaseStream_Impl {
    fn BaseStream(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_BaseStream(this: &Self::This, punkstream: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechCustomStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpeechBaseStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechCustomStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechCustomStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BaseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechCustomStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunkstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BaseStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_BaseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechCustomStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_BaseStream(this, ::windows_core::from_raw_borrowed(&punkstream)).into())
        }
        ISpeechCustomStream_Vtbl {
            base__: <ISpeechBaseStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BaseStream: BaseStream::<Identity, Impl, OFFSET>,
            putref_BaseStream: putref_BaseStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechDataKey_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetBinaryValue(this: &Self::This, valuename: &::windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetBinaryValue(this: &Self::This, valuename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetStringValue(this: &Self::This, valuename: &::windows_core::BSTR, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetStringValue(this: &Self::This, valuename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLongValue(this: &Self::This, valuename: &::windows_core::BSTR, value: i32) -> ::windows_core::Result<()>;
    fn GetLongValue(this: &Self::This, valuename: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn OpenKey(this: &Self::This, subkeyname: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechDataKey>;
    fn CreateKey(this: &Self::This, subkeyname: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechDataKey>;
    fn DeleteKey(this: &Self::This, subkeyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeleteValue(this: &Self::This, valuename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EnumKeys(this: &Self::This, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnumValues(this: &Self::This, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechDataKey {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechDataKey {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBinaryValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBinaryValue(this, ::core::mem::transmute(&valuename), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn GetBinaryValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBinaryValue(this, ::core::mem::transmute(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStringValue(this, ::core::mem::transmute(&valuename), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringValue(this, ::core::mem::transmute(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLongValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLongValue(this, ::core::mem::transmute(&valuename), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetLongValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLongValue(this, ::core::mem::transmute(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, subkey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenKey(this, ::core::mem::transmute(&subkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, subkey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateKey(this, ::core::mem::transmute(&subkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteKey(this, ::core::mem::transmute(&subkeyname)).into())
        }
        unsafe extern "system" fn DeleteValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteValue(this, ::core::mem::transmute(&valuename)).into())
        }
        unsafe extern "system" fn EnumKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, subkeyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumKeys(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subkeyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, valuename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumValues(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valuename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechDataKey_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBinaryValue: SetBinaryValue::<Identity, Impl, OFFSET>,
            GetBinaryValue: GetBinaryValue::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            SetLongValue: SetLongValue::<Identity, Impl, OFFSET>,
            GetLongValue: GetLongValue::<Identity, Impl, OFFSET>,
            OpenKey: OpenKey::<Identity, Impl, OFFSET>,
            CreateKey: CreateKey::<Identity, Impl, OFFSET>,
            DeleteKey: DeleteKey::<Identity, Impl, OFFSET>,
            DeleteValue: DeleteValue::<Identity, Impl, OFFSET>,
            EnumKeys: EnumKeys::<Identity, Impl, OFFSET>,
            EnumValues: EnumValues::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechFileStream_Impl: ::windows_core::BaseImpl + ISpeechBaseStream_Impl {
    fn Open(this: &Self::This, filename: &::windows_core::BSTR, filemode: SpeechStreamFileMode, doevents: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechFileStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpeechBaseStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechFileStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechFileStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, filemode: SpeechStreamFileMode, doevents: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&filemode), ::core::mem::transmute_copy(&doevents)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        ISpeechFileStream_Vtbl {
            base__: <ISpeechBaseStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRule_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Attributes(this: &Self::This) -> ::windows_core::Result<SpeechRuleAttributes>;
    fn InitialState(this: &Self::This) -> ::windows_core::Result<ISpeechGrammarRuleState>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddResource(this: &Self::This, resourcename: &::windows_core::BSTR, resourcevalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddState(this: &Self::This) -> ::windows_core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechGrammarRule {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechGrammarRule {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut SpeechRuleAttributes) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitialState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn AddResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, resourcevalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddResource(this, ::core::mem::transmute(&resourcename), ::core::mem::transmute(&resourcevalue)).into())
        }
        unsafe extern "system" fn AddState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechGrammarRule_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            InitialState: InitialState::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            AddState: AddState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRuleState_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Rule(this: &Self::This) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn Transitions(this: &Self::This) -> ::windows_core::Result<ISpeechGrammarRuleStateTransitions>;
    fn AddWordTransition(this: &Self::This, deststate: ::core::option::Option<&ISpeechGrammarRuleState>, words: &::windows_core::BSTR, separators: &::windows_core::BSTR, r#type: SpeechGrammarWordType, propertyname: &::windows_core::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::Result<()>;
    fn AddRuleTransition(this: &Self::This, destinationstate: ::core::option::Option<&ISpeechGrammarRuleState>, rule: ::core::option::Option<&ISpeechGrammarRule>, propertyname: &::windows_core::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::Result<()>;
    fn AddSpecialTransition(this: &Self::This, destinationstate: ::core::option::Option<&ISpeechGrammarRuleState>, r#type: SpeechSpecialTransitionType, propertyname: &::windows_core::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechGrammarRuleState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechGrammarRuleState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Rule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Rule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Transitions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Transitions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddWordTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deststate: *mut ::core::ffi::c_void, words: ::std::mem::MaybeUninit<::windows_core::BSTR>, separators: ::std::mem::MaybeUninit<::windows_core::BSTR>, r#type: SpeechGrammarWordType, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddWordTransition(this, ::windows_core::from_raw_borrowed(&deststate), ::core::mem::transmute(&words), ::core::mem::transmute(&separators), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into())
        }
        unsafe extern "system" fn AddRuleTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationstate: *mut ::core::ffi::c_void, rule: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRuleTransition(this, ::windows_core::from_raw_borrowed(&destinationstate), ::windows_core::from_raw_borrowed(&rule), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into())
        }
        unsafe extern "system" fn AddSpecialTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationstate: *mut ::core::ffi::c_void, r#type: SpeechSpecialTransitionType, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSpecialTransition(this, ::windows_core::from_raw_borrowed(&destinationstate), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into())
        }
        ISpeechGrammarRuleState_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Rule: Rule::<Identity, Impl, OFFSET>,
            Transitions: Transitions::<Identity, Impl, OFFSET>,
            AddWordTransition: AddWordTransition::<Identity, Impl, OFFSET>,
            AddRuleTransition: AddRuleTransition::<Identity, Impl, OFFSET>,
            AddSpecialTransition: AddSpecialTransition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRuleStateTransition_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<SpeechGrammarRuleStateTransitionType>;
    fn Text(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Rule(this: &Self::This) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn Weight(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PropertyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PropertyId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PropertyValue(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn NextState(this: &Self::This) -> ::windows_core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechGrammarRuleStateTransition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechGrammarRuleStateTransition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut SpeechGrammarRuleStateTransitionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Text<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Text(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Rule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Rule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Weight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, weight: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Weight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(weight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PropertyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PropertyId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NextState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechGrammarRuleStateTransition_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
            Rule: Rule::<Identity, Impl, OFFSET>,
            Weight: Weight::<Identity, Impl, OFFSET>,
            PropertyName: PropertyName::<Identity, Impl, OFFSET>,
            PropertyId: PropertyId::<Identity, Impl, OFFSET>,
            PropertyValue: PropertyValue::<Identity, Impl, OFFSET>,
            NextState: NextState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRuleStateTransitions_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechGrammarRuleStateTransition>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechGrammarRuleStateTransitions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechGrammarRuleStateTransitions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechGrammarRuleStateTransitions_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRules_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FindRule(this: &Self::This, rulenameorid: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Dynamic(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(this: &Self::This, rulename: &::windows_core::BSTR, attributes: SpeechRuleAttributes, ruleid: i32) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn CommitAndSave(this: &Self::This, errortext: *mut ::windows_core::BSTR, savestream: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechGrammarRules {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechGrammarRules {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rulenameorid: super::super::System::Variant::VARIANT, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindRule(this, ::core::mem::transmute(&rulenameorid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Dynamic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dynamic: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Dynamic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dynamic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rulename: ::std::mem::MaybeUninit<::windows_core::BSTR>, attributes: SpeechRuleAttributes, ruleid: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute(&rulename), ::core::mem::transmute_copy(&attributes), ::core::mem::transmute_copy(&ruleid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn CommitAndSave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errortext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, savestream: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitAndSave(this, ::core::mem::transmute_copy(&errortext), ::core::mem::transmute_copy(&savestream)).into())
        }
        ISpeechGrammarRules_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            FindRule: FindRule::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Dynamic: Dynamic::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            CommitAndSave: CommitAndSave::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexicon_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GenerationId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetWords(this: &Self::This, flags: SpeechLexiconType, generationid: *mut i32, words: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows_core::Result<()>;
    fn AddPronunciation(this: &Self::This, bstrword: &::windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddPronunciationByPhoneIds(this: &Self::This, bstrword: &::windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RemovePronunciation(this: &Self::This, bstrword: &::windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemovePronunciationByPhoneIds(this: &Self::This, bstrword: &::windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetPronunciations(this: &Self::This, bstrword: &::windows_core::BSTR, langid: i32, typeflags: SpeechLexiconType) -> ::windows_core::Result<ISpeechLexiconPronunciations>;
    fn GetGenerationChange(this: &Self::This, generationid: *mut i32, ppwords: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechLexicon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechLexicon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GenerationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, generationid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(generationid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: SpeechLexiconType, generationid: *mut i32, words: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWords(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&generationid), ::core::mem::transmute_copy(&words)).into())
        }
        unsafe extern "system" fn AddPronunciation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPronunciation(this, ::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute(&bstrpronunciation)).into())
        }
        unsafe extern "system" fn AddPronunciationByPhoneIds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPronunciationByPhoneIds(this, ::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&phoneids)).into())
        }
        unsafe extern "system" fn RemovePronunciation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePronunciation(this, ::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute(&bstrpronunciation)).into())
        }
        unsafe extern "system" fn RemovePronunciationByPhoneIds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePronunciationByPhoneIds(this, ::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&phoneids)).into())
        }
        unsafe extern "system" fn GetPronunciations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, typeflags: SpeechLexiconType, pppronunciations: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPronunciations(this, ::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&typeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppronunciations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, generationid: *mut i32, ppwords: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGenerationChange(this, ::core::mem::transmute_copy(&generationid), ::core::mem::transmute_copy(&ppwords)).into())
        }
        ISpeechLexicon_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GenerationId: GenerationId::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
            AddPronunciation: AddPronunciation::<Identity, Impl, OFFSET>,
            AddPronunciationByPhoneIds: AddPronunciationByPhoneIds::<Identity, Impl, OFFSET>,
            RemovePronunciation: RemovePronunciation::<Identity, Impl, OFFSET>,
            RemovePronunciationByPhoneIds: RemovePronunciationByPhoneIds::<Identity, Impl, OFFSET>,
            GetPronunciations: GetPronunciations::<Identity, Impl, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconPronunciation_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<SpeechLexiconType>;
    fn LangId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PartOfSpeech(this: &Self::This) -> ::windows_core::Result<SpeechPartOfSpeech>;
    fn PhoneIds(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Symbolic(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechLexiconPronunciation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechLexiconPronunciation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lexicontype: *mut SpeechLexiconType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lexicontype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LangId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LangId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(langid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PartOfSpeech<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partofspeech: *mut SpeechPartOfSpeech) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PartOfSpeech(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partofspeech, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PhoneIds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phoneids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PhoneIds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phoneids, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Symbolic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, symbolic: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Symbolic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(symbolic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechLexiconPronunciation_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            LangId: LangId::<Identity, Impl, OFFSET>,
            PartOfSpeech: PartOfSpeech::<Identity, Impl, OFFSET>,
            PhoneIds: PhoneIds::<Identity, Impl, OFFSET>,
            Symbolic: Symbolic::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconPronunciations_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechLexiconPronunciation>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechLexiconPronunciations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechLexiconPronunciations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pronunciation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pronunciation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechLexiconPronunciations_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconWord_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn LangId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Type(this: &Self::This) -> ::windows_core::Result<SpeechWordType>;
    fn Word(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Pronunciations(this: &Self::This) -> ::windows_core::Result<ISpeechLexiconPronunciations>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechLexiconWord {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechLexiconWord {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LangId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LangId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(langid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wordtype: *mut SpeechWordType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wordtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Word<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, word: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Word(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(word, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pronunciations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pronunciations: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Pronunciations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pronunciations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechLexiconWord_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LangId: LangId::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Word: Word::<Identity, Impl, OFFSET>,
            Pronunciations: Pronunciations::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconWords_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechLexiconWord>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechLexiconWords {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconWords_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechLexiconWords {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconWords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconWords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, word: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(word, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechLexiconWords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechLexiconWords_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechMMSysAudio_Impl: ::windows_core::BaseImpl + ISpeechAudio_Impl {
    fn DeviceId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDeviceId(this: &Self::This, deviceid: i32) -> ::windows_core::Result<()>;
    fn LineId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLineId(this: &Self::This, lineid: i32) -> ::windows_core::Result<()>;
    fn MMHandle(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechMMSysAudio {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpeechAudio);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechMMSysAudio {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeviceId(this, ::core::mem::transmute_copy(&deviceid)).into())
        }
        unsafe extern "system" fn LineId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lineid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LineId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLineId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lineid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLineId(this, ::core::mem::transmute_copy(&lineid)).into())
        }
        unsafe extern "system" fn MMHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MMHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(handle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechMMSysAudio_Vtbl {
            base__: <ISpeechAudio as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, Impl, OFFSET>,
            LineId: LineId::<Identity, Impl, OFFSET>,
            SetLineId: SetLineId::<Identity, Impl, OFFSET>,
            MMHandle: MMHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechMemoryStream_Impl: ::windows_core::BaseImpl + ISpeechBaseStream_Impl {
    fn SetData(this: &Self::This, data: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetData(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechMemoryStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpeechBaseStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechMemoryStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechMemoryStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechMemoryStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechMemoryStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechMemoryStream_Vtbl {
            base__: <ISpeechBaseStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetData: SetData::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechObjectToken_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DataKey(this: &Self::This) -> ::windows_core::Result<ISpeechDataKey>;
    fn Category(this: &Self::This) -> ::windows_core::Result<ISpeechObjectTokenCategory>;
    fn GetDescription(this: &Self::This, locale: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetId(this: &Self::This, id: &::windows_core::BSTR, categoryid: &::windows_core::BSTR, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetAttribute(this: &Self::This, attributename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateInstance(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, clscontext: SpeechTokenContext) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Remove(this: &Self::This, objectstorageclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetStorageFileName(this: &Self::This, objectstorageclsid: &::windows_core::BSTR, keyname: &::windows_core::BSTR, filename: &::windows_core::BSTR, folder: SpeechTokenShellFolder) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RemoveStorageFileName(this: &Self::This, objectstorageclsid: &::windows_core::BSTR, keyname: &::windows_core::BSTR, deletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsUISupported(this: &Self::This, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT, object: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisplayUI(this: &Self::This, hwnd: i32, title: &::windows_core::BSTR, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT, object: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn MatchesAttributes(this: &Self::This, attributes: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechObjectToken {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechObjectToken {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objectid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DataKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datakey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DataKey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datakey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Category<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Category(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(category, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locale: i32, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this, ::core::mem::transmute_copy(&locale)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::BSTR>, categoryid: ::std::mem::MaybeUninit<::windows_core::BSTR>, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&id), ::core::mem::transmute(&categoryid), ::core::mem::transmute_copy(&createifnotexist)).into())
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows_core::BSTR>, attributevalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttribute(this, ::core::mem::transmute(&attributename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributevalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, clscontext: SpeechTokenContext, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&clscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&objectstorageclsid)).into())
        }
        unsafe extern "system" fn GetStorageFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, keyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, folder: SpeechTokenShellFolder, filepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStorageFileName(this, ::core::mem::transmute(&objectstorageclsid), ::core::mem::transmute(&keyname), ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&folder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filepath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveStorageFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, keyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, deletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStorageFileName(this, ::core::mem::transmute(&objectstorageclsid), ::core::mem::transmute(&keyname), ::core::mem::transmute_copy(&deletefile)).into())
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT, object: *mut ::core::ffi::c_void, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUISupported(this, ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata), ::windows_core::from_raw_borrowed(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: i32, title: ::std::mem::MaybeUninit<::windows_core::BSTR>, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayUI(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&title), ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata), ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn MatchesAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, matches: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MatchesAttributes(this, ::core::mem::transmute(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechObjectToken_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            DataKey: DataKey::<Identity, Impl, OFFSET>,
            Category: Category::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetStorageFileName: GetStorageFileName::<Identity, Impl, OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            MatchesAttributes: MatchesAttributes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechObjectTokenCategory_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDefault(this: &Self::This, tokenid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Default(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetId(this: &Self::This, id: &::windows_core::BSTR, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetDataKey(this: &Self::This, location: SpeechDataKeyLocation) -> ::windows_core::Result<ISpeechDataKey>;
    fn EnumerateTokens(this: &Self::This, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechObjectTokenCategory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechObjectTokenCategory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tokenid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefault(this, ::core::mem::transmute(&tokenid)).into())
        }
        unsafe extern "system" fn Default<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tokenid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Default(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tokenid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::BSTR>, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&id), ::core::mem::transmute_copy(&createifnotexist)).into())
        }
        unsafe extern "system" fn GetDataKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: SpeechDataKeyLocation, datakey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataKey(this, ::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datakey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateTokens<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, tokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateTokens(this, ::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechObjectTokenCategory_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            SetDefault: SetDefault::<Identity, Impl, OFFSET>,
            Default: Default::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetDataKey: GetDataKey::<Identity, Impl, OFFSET>,
            EnumerateTokens: EnumerateTokens::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechObjectTokens_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechObjectToken>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechObjectTokens {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokens_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechObjectTokens {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokens_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokens_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, token: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechObjectTokens_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechObjectTokens_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhoneConverter_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLanguageId(this: &Self::This, languageid: i32) -> ::windows_core::Result<()>;
    fn PhoneToId(this: &Self::This, phonemes: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn IdToPhone(this: &Self::This, idarray: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhoneConverter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhoneConverter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LanguageId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LanguageId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLanguageId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguageId(this, ::core::mem::transmute_copy(&languageid)).into())
        }
        unsafe extern "system" fn PhoneToId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phonemes: ::std::mem::MaybeUninit<::windows_core::BSTR>, idarray: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PhoneToId(this, ::core::mem::transmute(&phonemes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(idarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IdToPhone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idarray: super::super::System::Variant::VARIANT, phonemes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IdToPhone(this, ::core::mem::transmute(&idarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phonemes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhoneConverter_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LanguageId: LanguageId::<Identity, Impl, OFFSET>,
            SetLanguageId: SetLanguageId::<Identity, Impl, OFFSET>,
            PhoneToId: PhoneToId::<Identity, Impl, OFFSET>,
            IdToPhone: IdToPhone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseAlternate_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RecoResult(this: &Self::This) -> ::windows_core::Result<ISpeechRecoResult>;
    fn StartElementInResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfElementsInResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PhraseInfo(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseInfo>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseAlternate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseAlternate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RecoResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recoresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecoResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recoresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartElementInResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startelement: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartElementInResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(startelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfElementsInResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfElementsInResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PhraseInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PhraseInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        ISpeechPhraseAlternate_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RecoResult: RecoResult::<Identity, Impl, OFFSET>,
            StartElementInResult: StartElementInResult::<Identity, Impl, OFFSET>,
            NumberOfElementsInResult: NumberOfElementsInResult::<Identity, Impl, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseAlternates_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechPhraseAlternate>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseAlternates {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseAlternates {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, phrasealternate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrasealternate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseAlternates_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseElement_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AudioTimeOffset(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AudioSizeTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AudioStreamOffset(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AudioSizeBytes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RetainedStreamOffset(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RetainedSizeBytes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DisplayText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LexicalForm(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Pronunciation(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DisplayAttributes(this: &Self::This) -> ::windows_core::Result<SpeechDisplayAttributes>;
    fn RequiredConfidence(this: &Self::This) -> ::windows_core::Result<SpeechEngineConfidence>;
    fn ActualConfidence(this: &Self::This) -> ::windows_core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(this: &Self::This) -> ::windows_core::Result<f32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AudioTimeOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiotimeoffset: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioTimeOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiotimeoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AudioSizeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioSizeTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiosizetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AudioStreamOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiostreamoffset: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioStreamOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiostreamoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AudioSizeBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiosizebytes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioSizeBytes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiosizebytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RetainedStreamOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retainedstreamoffset: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetainedStreamOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retainedstreamoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RetainedSizeBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetainedSizeBytes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retainedsizebytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displaytext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displaytext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LexicalForm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lexicalform: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LexicalForm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lexicalform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pronunciation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pronunciation: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Pronunciation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pronunciation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayAttributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequiredConfidence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredconfidence: *mut SpeechEngineConfidence) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequiredConfidence(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requiredconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActualConfidence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActualConfidence(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actualconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EngineConfidence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EngineConfidence(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(engineconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseElement_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AudioTimeOffset: AudioTimeOffset::<Identity, Impl, OFFSET>,
            AudioSizeTime: AudioSizeTime::<Identity, Impl, OFFSET>,
            AudioStreamOffset: AudioStreamOffset::<Identity, Impl, OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Identity, Impl, OFFSET>,
            RetainedStreamOffset: RetainedStreamOffset::<Identity, Impl, OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Identity, Impl, OFFSET>,
            DisplayText: DisplayText::<Identity, Impl, OFFSET>,
            LexicalForm: LexicalForm::<Identity, Impl, OFFSET>,
            Pronunciation: Pronunciation::<Identity, Impl, OFFSET>,
            DisplayAttributes: DisplayAttributes::<Identity, Impl, OFFSET>,
            RequiredConfidence: RequiredConfidence::<Identity, Impl, OFFSET>,
            ActualConfidence: ActualConfidence::<Identity, Impl, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseElements_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechPhraseElement>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseElements {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElements_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseElements {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseElements_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GrammarId(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn StartTime(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AudioStreamPosition(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AudioSizeBytes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RetainedSizeBytes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AudioSizeTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Rule(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseRule>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseProperties>;
    fn Elements(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseElements>;
    fn Replacements(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseReplacements>;
    fn EngineId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnginePrivateData(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SaveToMemory(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetText(this: &Self::This, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDisplayAttributes(this: &Self::This, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<SpeechDisplayAttributes>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LanguageId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LanguageId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GrammarId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grammarid: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GrammarId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(grammarid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starttime: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(starttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AudioStreamPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiostreamposition: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioStreamPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiostreamposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AudioSizeBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paudiosizebytes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioSizeBytes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paudiosizebytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RetainedSizeBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetainedSizeBytes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retainedsizebytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AudioSizeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioSizeTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiosizetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Rule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Rule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Elements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elements: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Elements(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(elements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Replacements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, replacements: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Replacements(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(replacements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EngineId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, engineidguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EngineId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(engineidguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnginePrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, privatedata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnginePrivateData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(privatedata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SaveToMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phraseblock: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveToMemory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetText(this, ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&usereplacements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL, displayattributes: *mut SpeechDisplayAttributes) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayAttributes(this, ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&usereplacements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LanguageId: LanguageId::<Identity, Impl, OFFSET>,
            GrammarId: GrammarId::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            AudioStreamPosition: AudioStreamPosition::<Identity, Impl, OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Identity, Impl, OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Identity, Impl, OFFSET>,
            AudioSizeTime: AudioSizeTime::<Identity, Impl, OFFSET>,
            Rule: Rule::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Elements: Elements::<Identity, Impl, OFFSET>,
            Replacements: Replacements::<Identity, Impl, OFFSET>,
            EngineId: EngineId::<Identity, Impl, OFFSET>,
            EnginePrivateData: EnginePrivateData::<Identity, Impl, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetDisplayAttributes: GetDisplayAttributes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseInfoBuilder_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RestorePhraseFromMemory(this: &Self::This, phraseinmemory: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISpeechPhraseInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseInfoBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfoBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseInfoBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RestorePhraseFromMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseInfoBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phraseinmemory: *const super::super::System::Variant::VARIANT, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RestorePhraseFromMemory(this, ::core::mem::transmute_copy(&phraseinmemory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseInfoBuilder_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RestorePhraseFromMemory: RestorePhraseFromMemory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseProperties_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechPhraseProperty>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, property: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseProperties_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseProperty_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Value(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn FirstElement(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfElements(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EngineConfidence(this: &Self::This) -> ::windows_core::Result<f32>;
    fn Confidence(this: &Self::This) -> ::windows_core::Result<SpeechEngineConfidence>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseProperty>;
    fn Children(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FirstElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirstElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(firstelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfElements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfElements(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EngineConfidence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, confidence: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EngineConfidence(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(confidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Confidence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, confidence: *mut SpeechEngineConfidence) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Confidence(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(confidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parentproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Children(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseProperty_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            FirstElement: FirstElement::<Identity, Impl, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, Impl, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, Impl, OFFSET>,
            Confidence: Confidence::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseReplacement_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DisplayAttributes(this: &Self::This) -> ::windows_core::Result<SpeechDisplayAttributes>;
    fn Text(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FirstElement(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfElements(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseReplacement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseReplacement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisplayAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayAttributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Text<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Text(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FirstElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirstElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(firstelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfElements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfElements(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseReplacement_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisplayAttributes: DisplayAttributes::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
            FirstElement: FirstElement::<Identity, Impl, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseReplacements_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechPhraseReplacement>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseReplacements {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseReplacements {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, reps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseReplacements_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseRule_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FirstElement(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfElements(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseRule>;
    fn Children(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseRules>;
    fn Confidence(this: &Self::This) -> ::windows_core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(this: &Self::This) -> ::windows_core::Result<f32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseRule {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseRule {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FirstElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirstElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(firstelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfElements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfElements(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Children(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Confidence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Confidence(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actualconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EngineConfidence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EngineConfidence(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(engineconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseRule_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            FirstElement: FirstElement::<Identity, Impl, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            Confidence: Confidence::<Identity, Impl, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseRules_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<ISpeechPhraseRule>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechPhraseRules {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRules_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechPhraseRules {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechPhraseRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechPhraseRules_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoContext_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Recognizer(this: &Self::This) -> ::windows_core::Result<ISpeechRecognizer>;
    fn AudioInputInterferenceStatus(this: &Self::This) -> ::windows_core::Result<SpeechInterference>;
    fn RequestedUIType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn putref_Voice(this: &Self::This, voice: ::core::option::Option<&ISpeechVoice>) -> ::windows_core::Result<()>;
    fn Voice(this: &Self::This) -> ::windows_core::Result<ISpeechVoice>;
    fn SetAllowVoiceFormatMatchingOnNextSet(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowVoiceFormatMatchingOnNextSet(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetVoicePurgeEvent(this: &Self::This, eventinterest: SpeechRecoEvents) -> ::windows_core::Result<()>;
    fn VoicePurgeEvent(this: &Self::This) -> ::windows_core::Result<SpeechRecoEvents>;
    fn SetEventInterests(this: &Self::This, eventinterest: SpeechRecoEvents) -> ::windows_core::Result<()>;
    fn EventInterests(this: &Self::This) -> ::windows_core::Result<SpeechRecoEvents>;
    fn SetCmdMaxAlternates(this: &Self::This, maxalternates: i32) -> ::windows_core::Result<()>;
    fn CmdMaxAlternates(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetState(this: &Self::This, state: SpeechRecoContextState) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<SpeechRecoContextState>;
    fn SetRetainedAudio(this: &Self::This, option: SpeechRetainedAudioOptions) -> ::windows_core::Result<()>;
    fn RetainedAudio(this: &Self::This) -> ::windows_core::Result<SpeechRetainedAudioOptions>;
    fn putref_RetainedAudioFormat(this: &Self::This, format: ::core::option::Option<&ISpeechAudioFormat>) -> ::windows_core::Result<()>;
    fn RetainedAudioFormat(this: &Self::This) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateGrammar(this: &Self::This, grammarid: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISpeechRecoGrammar>;
    fn CreateResultFromMemory(this: &Self::This, resultblock: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISpeechRecoResult>;
    fn Bookmark(this: &Self::This, options: SpeechBookmarkOptions, streampos: &super::super::System::Variant::VARIANT, bookmarkid: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetAdaptationData(this: &Self::This, adaptationstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechRecoContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechRecoContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Recognizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recognizer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recognizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AudioInputInterferenceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interference: *mut SpeechInterference) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioInputInterferenceStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestedUIType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uitype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestedUIType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uitype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_Voice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_Voice(this, ::windows_core::from_raw_borrowed(&voice)).into())
        }
        unsafe extern "system" fn Voice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Voice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(voice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowVoiceFormatMatchingOnNextSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowVoiceFormatMatchingOnNextSet(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowVoiceFormatMatchingOnNextSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pallow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowVoiceFormatMatchingOnNextSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pallow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVoicePurgeEvent(this, ::core::mem::transmute_copy(&eventinterest)).into())
        }
        unsafe extern "system" fn VoicePurgeEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VoicePurgeEvent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventinterest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventInterests<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventInterests(this, ::core::mem::transmute_copy(&eventinterest)).into())
        }
        unsafe extern "system" fn EventInterests<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventInterests(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventinterest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCmdMaxAlternates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxalternates: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCmdMaxAlternates(this, ::core::mem::transmute_copy(&maxalternates)).into())
        }
        unsafe extern "system" fn CmdMaxAlternates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxalternates: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CmdMaxAlternates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxalternates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: SpeechRecoContextState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecoContextState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRetainedAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: SpeechRetainedAudioOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRetainedAudio(this, ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn RetainedAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: *mut SpeechRetainedAudioOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetainedAudio(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(option, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_RetainedAudioFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_RetainedAudioFormat(this, ::windows_core::from_raw_borrowed(&format)).into())
        }
        unsafe extern "system" fn RetainedAudioFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetainedAudioFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn CreateGrammar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grammarid: super::super::System::Variant::VARIANT, grammar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGrammar(this, ::core::mem::transmute(&grammarid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(grammar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateResultFromMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resultblock: *const super::super::System::Variant::VARIANT, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateResultFromMemory(this, ::core::mem::transmute_copy(&resultblock)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Bookmark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: SpeechBookmarkOptions, streampos: super::super::System::Variant::VARIANT, bookmarkid: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Bookmark(this, ::core::mem::transmute_copy(&options), ::core::mem::transmute(&streampos), ::core::mem::transmute(&bookmarkid)).into())
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adaptationstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAdaptationData(this, ::core::mem::transmute(&adaptationstring)).into())
        }
        ISpeechRecoContext_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Recognizer: Recognizer::<Identity, Impl, OFFSET>,
            AudioInputInterferenceStatus: AudioInputInterferenceStatus::<Identity, Impl, OFFSET>,
            RequestedUIType: RequestedUIType::<Identity, Impl, OFFSET>,
            putref_Voice: putref_Voice::<Identity, Impl, OFFSET>,
            Voice: Voice::<Identity, Impl, OFFSET>,
            SetAllowVoiceFormatMatchingOnNextSet: SetAllowVoiceFormatMatchingOnNextSet::<Identity, Impl, OFFSET>,
            AllowVoiceFormatMatchingOnNextSet: AllowVoiceFormatMatchingOnNextSet::<Identity, Impl, OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Identity, Impl, OFFSET>,
            VoicePurgeEvent: VoicePurgeEvent::<Identity, Impl, OFFSET>,
            SetEventInterests: SetEventInterests::<Identity, Impl, OFFSET>,
            EventInterests: EventInterests::<Identity, Impl, OFFSET>,
            SetCmdMaxAlternates: SetCmdMaxAlternates::<Identity, Impl, OFFSET>,
            CmdMaxAlternates: CmdMaxAlternates::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetRetainedAudio: SetRetainedAudio::<Identity, Impl, OFFSET>,
            RetainedAudio: RetainedAudio::<Identity, Impl, OFFSET>,
            putref_RetainedAudioFormat: putref_RetainedAudioFormat::<Identity, Impl, OFFSET>,
            RetainedAudioFormat: RetainedAudioFormat::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            CreateGrammar: CreateGrammar::<Identity, Impl, OFFSET>,
            CreateResultFromMemory: CreateResultFromMemory::<Identity, Impl, OFFSET>,
            Bookmark: Bookmark::<Identity, Impl, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoGrammar_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Id(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn RecoContext(this: &Self::This) -> ::windows_core::Result<ISpeechRecoContext>;
    fn SetState(this: &Self::This, state: SpeechGrammarState) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<SpeechGrammarState>;
    fn Rules(this: &Self::This) -> ::windows_core::Result<ISpeechGrammarRules>;
    fn Reset(this: &Self::This, newlanguage: i32) -> ::windows_core::Result<()>;
    fn CmdLoadFromFile(this: &Self::This, filename: &::windows_core::BSTR, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdLoadFromObject(this: &Self::This, classid: &::windows_core::BSTR, grammarname: &::windows_core::BSTR, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdLoadFromResource(this: &Self::This, hmodule: i32, resourcename: &super::super::System::Variant::VARIANT, resourcetype: &super::super::System::Variant::VARIANT, languageid: i32, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdLoadFromMemory(this: &Self::This, grammardata: &super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdLoadFromProprietaryGrammar(this: &Self::This, proprietaryguid: &::windows_core::BSTR, proprietarystring: &::windows_core::BSTR, proprietarydata: &super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdSetRuleState(this: &Self::This, name: &::windows_core::BSTR, state: SpeechRuleState) -> ::windows_core::Result<()>;
    fn CmdSetRuleIdState(this: &Self::This, ruleid: i32, state: SpeechRuleState) -> ::windows_core::Result<()>;
    fn DictationLoad(this: &Self::This, topicname: &::windows_core::BSTR, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn DictationUnload(this: &Self::This) -> ::windows_core::Result<()>;
    fn DictationSetState(this: &Self::This, state: SpeechRuleState) -> ::windows_core::Result<()>;
    fn SetWordSequenceData(this: &Self::This, text: &::windows_core::BSTR, textlength: i32, info: ::core::option::Option<&ISpeechTextSelectionInformation>) -> ::windows_core::Result<()>;
    fn SetTextSelection(this: &Self::This, info: ::core::option::Option<&ISpeechTextSelectionInformation>) -> ::windows_core::Result<()>;
    fn IsPronounceable(this: &Self::This, word: &::windows_core::BSTR) -> ::windows_core::Result<SpeechWordPronounceable>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechRecoGrammar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechRecoGrammar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RecoContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecoContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recocontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: SpeechGrammarState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut SpeechGrammarState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Rules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Rules(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newlanguage: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this, ::core::mem::transmute_copy(&newlanguage)).into())
        }
        unsafe extern "system" fn CmdLoadFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CmdLoadFromFile(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&loadoption)).into())
        }
        unsafe extern "system" fn CmdLoadFromObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: ::std::mem::MaybeUninit<::windows_core::BSTR>, grammarname: ::std::mem::MaybeUninit<::windows_core::BSTR>, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CmdLoadFromObject(this, ::core::mem::transmute(&classid), ::core::mem::transmute(&grammarname), ::core::mem::transmute_copy(&loadoption)).into())
        }
        unsafe extern "system" fn CmdLoadFromResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmodule: i32, resourcename: super::super::System::Variant::VARIANT, resourcetype: super::super::System::Variant::VARIANT, languageid: i32, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CmdLoadFromResource(this, ::core::mem::transmute_copy(&hmodule), ::core::mem::transmute(&resourcename), ::core::mem::transmute(&resourcetype), ::core::mem::transmute_copy(&languageid), ::core::mem::transmute_copy(&loadoption)).into())
        }
        unsafe extern "system" fn CmdLoadFromMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grammardata: super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CmdLoadFromMemory(this, ::core::mem::transmute(&grammardata), ::core::mem::transmute_copy(&loadoption)).into())
        }
        unsafe extern "system" fn CmdLoadFromProprietaryGrammar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, proprietaryguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, proprietarystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, proprietarydata: super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CmdLoadFromProprietaryGrammar(this, ::core::mem::transmute(&proprietaryguid), ::core::mem::transmute(&proprietarystring), ::core::mem::transmute(&proprietarydata), ::core::mem::transmute_copy(&loadoption)).into())
        }
        unsafe extern "system" fn CmdSetRuleState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, state: SpeechRuleState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CmdSetRuleState(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn CmdSetRuleIdState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ruleid: i32, state: SpeechRuleState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CmdSetRuleIdState(this, ::core::mem::transmute_copy(&ruleid), ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn DictationLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, topicname: ::std::mem::MaybeUninit<::windows_core::BSTR>, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DictationLoad(this, ::core::mem::transmute(&topicname), ::core::mem::transmute_copy(&loadoption)).into())
        }
        unsafe extern "system" fn DictationUnload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DictationUnload(this).into())
        }
        unsafe extern "system" fn DictationSetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: SpeechRuleState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DictationSetState(this, ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>, textlength: i32, info: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWordSequenceData(this, ::core::mem::transmute(&text), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&info)).into())
        }
        unsafe extern "system" fn SetTextSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, info: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextSelection(this, ::windows_core::from_raw_borrowed(&info)).into())
        }
        unsafe extern "system" fn IsPronounceable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, word: ::std::mem::MaybeUninit<::windows_core::BSTR>, wordpronounceable: *mut SpeechWordPronounceable) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPronounceable(this, ::core::mem::transmute(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wordpronounceable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechRecoGrammar_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            RecoContext: RecoContext::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            CmdLoadFromFile: CmdLoadFromFile::<Identity, Impl, OFFSET>,
            CmdLoadFromObject: CmdLoadFromObject::<Identity, Impl, OFFSET>,
            CmdLoadFromResource: CmdLoadFromResource::<Identity, Impl, OFFSET>,
            CmdLoadFromMemory: CmdLoadFromMemory::<Identity, Impl, OFFSET>,
            CmdLoadFromProprietaryGrammar: CmdLoadFromProprietaryGrammar::<Identity, Impl, OFFSET>,
            CmdSetRuleState: CmdSetRuleState::<Identity, Impl, OFFSET>,
            CmdSetRuleIdState: CmdSetRuleIdState::<Identity, Impl, OFFSET>,
            DictationLoad: DictationLoad::<Identity, Impl, OFFSET>,
            DictationUnload: DictationUnload::<Identity, Impl, OFFSET>,
            DictationSetState: DictationSetState::<Identity, Impl, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, Impl, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, Impl, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResult_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(this: &Self::This) -> ::windows_core::Result<ISpeechRecoContext>;
    fn Times(this: &Self::This) -> ::windows_core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(this: &Self::This, format: ::core::option::Option<&ISpeechAudioFormat>) -> ::windows_core::Result<()>;
    fn AudioFormat(this: &Self::This) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseInfo>;
    fn Alternates(this: &Self::This, requestcount: i32, startelement: i32, elements: i32) -> ::windows_core::Result<ISpeechPhraseAlternates>;
    fn Audio(this: &Self::This, startelement: i32, elements: i32) -> ::windows_core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(this: &Self::This, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows_core::Result<i32>;
    fn SaveToMemory(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DiscardResultInfo(this: &Self::This, valuetypes: SpeechDiscardType) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechRecoResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechRecoResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RecoContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecoContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recocontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Times<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, times: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Times(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(times, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AudioFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AudioFormat(this, ::windows_core::from_raw_borrowed(&format)).into())
        }
        unsafe extern "system" fn AudioFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PhraseInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PhraseInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Alternates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Alternates(this, ::core::mem::transmute_copy(&requestcount), ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(alternates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Audio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Audio(this, ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpeakAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpeakAudio(this, ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SaveToMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveToMemory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DiscardResultInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardResultInfo(this, ::core::mem::transmute_copy(&valuetypes)).into())
        }
        ISpeechRecoResult_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RecoContext: RecoContext::<Identity, Impl, OFFSET>,
            Times: Times::<Identity, Impl, OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Identity, Impl, OFFSET>,
            AudioFormat: AudioFormat::<Identity, Impl, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, Impl, OFFSET>,
            Alternates: Alternates::<Identity, Impl, OFFSET>,
            Audio: Audio::<Identity, Impl, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, Impl, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, Impl, OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResult2_Impl: ::windows_core::BaseImpl + ISpeechRecoResult_Impl {
    fn SetTextFeedback(this: &Self::This, feedback: &::windows_core::BSTR, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechRecoResult2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpeechRecoResult);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechRecoResult2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTextFeedback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResult2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedback: ::std::mem::MaybeUninit<::windows_core::BSTR>, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextFeedback(this, ::core::mem::transmute(&feedback), ::core::mem::transmute_copy(&wassuccessful)).into())
        }
        ISpeechRecoResult2_Vtbl { base__: <ISpeechRecoResult as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetTextFeedback: SetTextFeedback::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResultDispatch_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(this: &Self::This) -> ::windows_core::Result<ISpeechRecoContext>;
    fn Times(this: &Self::This) -> ::windows_core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(this: &Self::This, format: ::core::option::Option<&ISpeechAudioFormat>) -> ::windows_core::Result<()>;
    fn AudioFormat(this: &Self::This) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(this: &Self::This) -> ::windows_core::Result<ISpeechPhraseInfo>;
    fn Alternates(this: &Self::This, requestcount: i32, startelement: i32, elements: i32) -> ::windows_core::Result<ISpeechPhraseAlternates>;
    fn Audio(this: &Self::This, startelement: i32, elements: i32) -> ::windows_core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(this: &Self::This, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows_core::Result<i32>;
    fn SaveToMemory(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DiscardResultInfo(this: &Self::This, valuetypes: SpeechDiscardType) -> ::windows_core::Result<()>;
    fn GetXMLResult(this: &Self::This, options: SPXMLRESULTOPTIONS) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetXMLErrorInfo(this: &Self::This, linenumber: *mut i32, scriptline: *mut ::windows_core::BSTR, source: *mut ::windows_core::BSTR, description: *mut ::windows_core::BSTR, resultcode: *mut ::windows_core::HRESULT, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetTextFeedback(this: &Self::This, feedback: &::windows_core::BSTR, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechRecoResultDispatch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechRecoResultDispatch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RecoContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecoContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recocontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Times<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, times: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Times(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(times, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AudioFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AudioFormat(this, ::windows_core::from_raw_borrowed(&format)).into())
        }
        unsafe extern "system" fn AudioFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PhraseInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PhraseInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Alternates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Alternates(this, ::core::mem::transmute_copy(&requestcount), ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(alternates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Audio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Audio(this, ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpeakAudio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpeakAudio(this, ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SaveToMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveToMemory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DiscardResultInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardResultInfo(this, ::core::mem::transmute_copy(&valuetypes)).into())
        }
        unsafe extern "system" fn GetXMLResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXMLResult(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, source: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, resultcode: *mut ::windows_core::HRESULT, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetXMLErrorInfo(this, ::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)).into())
        }
        unsafe extern "system" fn SetTextFeedback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedback: ::std::mem::MaybeUninit<::windows_core::BSTR>, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextFeedback(this, ::core::mem::transmute(&feedback), ::core::mem::transmute_copy(&wassuccessful)).into())
        }
        ISpeechRecoResultDispatch_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RecoContext: RecoContext::<Identity, Impl, OFFSET>,
            Times: Times::<Identity, Impl, OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Identity, Impl, OFFSET>,
            AudioFormat: AudioFormat::<Identity, Impl, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, Impl, OFFSET>,
            Alternates: Alternates::<Identity, Impl, OFFSET>,
            Audio: Audio::<Identity, Impl, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, Impl, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, Impl, OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Identity, Impl, OFFSET>,
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
            SetTextFeedback: SetTextFeedback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResultTimes_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StreamTime(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Length(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn TickCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn OffsetFromStart(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechRecoResultTimes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechRecoResultTimes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StreamTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StreamTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(time, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TickCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tickcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TickCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tickcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OffsetFromStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OffsetFromStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(offsetfromstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechRecoResultTimes_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StreamTime: StreamTime::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            OffsetFromStart: OffsetFromStart::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecognizer_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn putref_Recognizer(this: &Self::This, recognizer: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn Recognizer(this: &Self::This) -> ::windows_core::Result<ISpeechObjectToken>;
    fn SetAllowAudioInputFormatChangesOnNextSet(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowAudioInputFormatChangesOnNextSet(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn putref_AudioInput(this: &Self::This, audioinput: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn AudioInput(this: &Self::This) -> ::windows_core::Result<ISpeechObjectToken>;
    fn putref_AudioInputStream(this: &Self::This, audioinputstream: ::core::option::Option<&ISpeechBaseStream>) -> ::windows_core::Result<()>;
    fn AudioInputStream(this: &Self::This) -> ::windows_core::Result<ISpeechBaseStream>;
    fn IsShared(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetState(this: &Self::This, state: SpeechRecognizerState) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<SpeechRecognizerState>;
    fn Status(this: &Self::This) -> ::windows_core::Result<ISpeechRecognizerStatus>;
    fn putref_Profile(this: &Self::This, profile: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn Profile(this: &Self::This) -> ::windows_core::Result<ISpeechObjectToken>;
    fn EmulateRecognition(this: &Self::This, textelements: &super::super::System::Variant::VARIANT, elementdisplayattributes: *const super::super::System::Variant::VARIANT, languageid: i32) -> ::windows_core::Result<()>;
    fn CreateRecoContext(this: &Self::This) -> ::windows_core::Result<ISpeechRecoContext>;
    fn GetFormat(this: &Self::This, r#type: SpeechFormatType) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn SetPropertyNumber(this: &Self::This, name: &::windows_core::BSTR, value: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetPropertyNumber(this: &Self::This, name: &::windows_core::BSTR, value: *mut i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetPropertyString(this: &Self::This, name: &::windows_core::BSTR, value: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetPropertyString(this: &Self::This, name: &::windows_core::BSTR, value: *mut ::windows_core::BSTR, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsUISupported(this: &Self::This, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisplayUI(this: &Self::This, hwndparent: i32, title: &::windows_core::BSTR, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetRecognizers(this: &Self::This, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
    fn GetAudioInputs(this: &Self::This, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
    fn GetProfiles(this: &Self::This, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechRecognizer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechRecognizer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn putref_Recognizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_Recognizer(this, ::windows_core::from_raw_borrowed(&recognizer)).into())
        }
        unsafe extern "system" fn Recognizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Recognizer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recognizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowAudioInputFormatChangesOnNextSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowAudioInputFormatChangesOnNextSet(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowAudioInputFormatChangesOnNextSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowAudioInputFormatChangesOnNextSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AudioInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audioinput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AudioInput(this, ::windows_core::from_raw_borrowed(&audioinput)).into())
        }
        unsafe extern "system" fn AudioInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audioinput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioInput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioinput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AudioInputStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audioinputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AudioInputStream(this, ::windows_core::from_raw_borrowed(&audioinputstream)).into())
        }
        unsafe extern "system" fn AudioInputStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audioinputstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioInputStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioinputstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsShared<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shared: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsShared(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shared, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: SpeechRecognizerState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecognizerState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_Profile(this, ::windows_core::from_raw_borrowed(&profile)).into())
        }
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(profile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textelements: super::super::System::Variant::VARIANT, elementdisplayattributes: *const super::super::System::Variant::VARIANT, languageid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EmulateRecognition(this, ::core::mem::transmute(&textelements), ::core::mem::transmute_copy(&elementdisplayattributes), ::core::mem::transmute_copy(&languageid)).into())
        }
        unsafe extern "system" fn CreateRecoContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRecoContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: SpeechFormatType, format: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormat(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPropertyNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetPropertyNumber(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyNumber(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&supported)).into())
        }
        unsafe extern "system" fn SetPropertyString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetPropertyString(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyString(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&supported)).into())
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUISupported(this, ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::std::mem::MaybeUninit<::windows_core::BSTR>, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayUI(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&title), ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)).into())
        }
        unsafe extern "system" fn GetRecognizers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecognizers(this, ::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAudioInputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAudioInputs(this, ::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProfiles(this, ::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechRecognizer_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            putref_Recognizer: putref_Recognizer::<Identity, Impl, OFFSET>,
            Recognizer: Recognizer::<Identity, Impl, OFFSET>,
            SetAllowAudioInputFormatChangesOnNextSet: SetAllowAudioInputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            AllowAudioInputFormatChangesOnNextSet: AllowAudioInputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            putref_AudioInput: putref_AudioInput::<Identity, Impl, OFFSET>,
            AudioInput: AudioInput::<Identity, Impl, OFFSET>,
            putref_AudioInputStream: putref_AudioInputStream::<Identity, Impl, OFFSET>,
            AudioInputStream: AudioInputStream::<Identity, Impl, OFFSET>,
            IsShared: IsShared::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            putref_Profile: putref_Profile::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, Impl, OFFSET>,
            CreateRecoContext: CreateRecoContext::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            SetPropertyNumber: SetPropertyNumber::<Identity, Impl, OFFSET>,
            GetPropertyNumber: GetPropertyNumber::<Identity, Impl, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, Impl, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            GetRecognizers: GetRecognizers::<Identity, Impl, OFFSET>,
            GetAudioInputs: GetAudioInputs::<Identity, Impl, OFFSET>,
            GetProfiles: GetProfiles::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecognizerStatus_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AudioStatus(this: &Self::This) -> ::windows_core::Result<ISpeechAudioStatus>;
    fn CurrentStreamPosition(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CurrentStreamNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfActiveRules(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ClsidEngine(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SupportedLanguages(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechRecognizerStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechRecognizerStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AudioStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiostatus: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiostatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentStreamPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcurrentstreampos: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentStreamPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcurrentstreampos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentStreamNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentStreamNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfActiveRules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberofactiverules: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfActiveRules(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofactiverules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClsidEngine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidengine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClsidEngine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clsidengine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedLanguages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedlanguages: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedLanguages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedlanguages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechRecognizerStatus_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AudioStatus: AudioStatus::<Identity, Impl, OFFSET>,
            CurrentStreamPosition: CurrentStreamPosition::<Identity, Impl, OFFSET>,
            CurrentStreamNumber: CurrentStreamNumber::<Identity, Impl, OFFSET>,
            NumberOfActiveRules: NumberOfActiveRules::<Identity, Impl, OFFSET>,
            ClsidEngine: ClsidEngine::<Identity, Impl, OFFSET>,
            SupportedLanguages: SupportedLanguages::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechResourceLoader_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn LoadResource(this: &Self::This, bstrresourceuri: &::windows_core::BSTR, falwaysreload: super::super::Foundation::VARIANT_BOOL, pstream: *mut ::core::option::Option<::windows_core::IUnknown>, pbstrmimetype: *mut ::windows_core::BSTR, pfmodified: *mut super::super::Foundation::VARIANT_BOOL, pbstrredirecturl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetLocalCopy(this: &Self::This, bstrresourceuri: &::windows_core::BSTR, pbstrlocalpath: *mut ::windows_core::BSTR, pbstrmimetype: *mut ::windows_core::BSTR, pbstrredirecturl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReleaseLocalCopy(this: &Self::This, pbstrlocalpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechResourceLoader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechResourceLoader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechResourceLoader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechResourceLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, falwaysreload: super::super::Foundation::VARIANT_BOOL, pstream: *mut *mut ::core::ffi::c_void, pbstrmimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfmodified: *mut super::super::Foundation::VARIANT_BOOL, pbstrredirecturl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadResource(this, ::core::mem::transmute(&bstrresourceuri), ::core::mem::transmute_copy(&falwaysreload), ::core::mem::transmute_copy(&pstream), ::core::mem::transmute_copy(&pbstrmimetype), ::core::mem::transmute_copy(&pfmodified), ::core::mem::transmute_copy(&pbstrredirecturl)).into())
        }
        unsafe extern "system" fn GetLocalCopy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechResourceLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrlocalpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrmimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrredirecturl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocalCopy(this, ::core::mem::transmute(&bstrresourceuri), ::core::mem::transmute_copy(&pbstrlocalpath), ::core::mem::transmute_copy(&pbstrmimetype), ::core::mem::transmute_copy(&pbstrredirecturl)).into())
        }
        unsafe extern "system" fn ReleaseLocalCopy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechResourceLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlocalpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseLocalCopy(this, ::core::mem::transmute(&pbstrlocalpath)).into())
        }
        ISpeechResourceLoader_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadResource: LoadResource::<Identity, Impl, OFFSET>,
            GetLocalCopy: GetLocalCopy::<Identity, Impl, OFFSET>,
            ReleaseLocalCopy: ReleaseLocalCopy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechTextSelectionInformation_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetActiveOffset(this: &Self::This, activeoffset: i32) -> ::windows_core::Result<()>;
    fn ActiveOffset(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetActiveLength(this: &Self::This, activelength: i32) -> ::windows_core::Result<()>;
    fn ActiveLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSelectionOffset(this: &Self::This, selectionoffset: i32) -> ::windows_core::Result<()>;
    fn SelectionOffset(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSelectionLength(this: &Self::This, selectionlength: i32) -> ::windows_core::Result<()>;
    fn SelectionLength(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechTextSelectionInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechTextSelectionInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetActiveOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activeoffset: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActiveOffset(this, ::core::mem::transmute_copy(&activeoffset)).into())
        }
        unsafe extern "system" fn ActiveOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activeoffset: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActiveOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activeoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetActiveLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activelength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActiveLength(this, ::core::mem::transmute_copy(&activelength)).into())
        }
        unsafe extern "system" fn ActiveLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activelength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActiveLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activelength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSelectionOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectionoffset: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelectionOffset(this, ::core::mem::transmute_copy(&selectionoffset)).into())
        }
        unsafe extern "system" fn SelectionOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectionoffset: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectionOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectionoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSelectionLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectionlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelectionLength(this, ::core::mem::transmute_copy(&selectionlength)).into())
        }
        unsafe extern "system" fn SelectionLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectionlength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectionLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectionlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechTextSelectionInformation_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetActiveOffset: SetActiveOffset::<Identity, Impl, OFFSET>,
            ActiveOffset: ActiveOffset::<Identity, Impl, OFFSET>,
            SetActiveLength: SetActiveLength::<Identity, Impl, OFFSET>,
            ActiveLength: ActiveLength::<Identity, Impl, OFFSET>,
            SetSelectionOffset: SetSelectionOffset::<Identity, Impl, OFFSET>,
            SelectionOffset: SelectionOffset::<Identity, Impl, OFFSET>,
            SetSelectionLength: SetSelectionLength::<Identity, Impl, OFFSET>,
            SelectionLength: SelectionLength::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechVoice_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Status(this: &Self::This) -> ::windows_core::Result<ISpeechVoiceStatus>;
    fn Voice(this: &Self::This) -> ::windows_core::Result<ISpeechObjectToken>;
    fn putref_Voice(this: &Self::This, voice: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn AudioOutput(this: &Self::This) -> ::windows_core::Result<ISpeechObjectToken>;
    fn putref_AudioOutput(this: &Self::This, audiooutput: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn AudioOutputStream(this: &Self::This) -> ::windows_core::Result<ISpeechBaseStream>;
    fn putref_AudioOutputStream(this: &Self::This, audiooutputstream: ::core::option::Option<&ISpeechBaseStream>) -> ::windows_core::Result<()>;
    fn Rate(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRate(this: &Self::This, rate: i32) -> ::windows_core::Result<()>;
    fn Volume(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetVolume(this: &Self::This, volume: i32) -> ::windows_core::Result<()>;
    fn SetAllowAudioOutputFormatChangesOnNextSet(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowAudioOutputFormatChangesOnNextSet(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EventInterests(this: &Self::This) -> ::windows_core::Result<SpeechVoiceEvents>;
    fn SetEventInterests(this: &Self::This, eventinterestflags: SpeechVoiceEvents) -> ::windows_core::Result<()>;
    fn SetPriority(this: &Self::This, priority: SpeechVoicePriority) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<SpeechVoicePriority>;
    fn SetAlertBoundary(this: &Self::This, boundary: SpeechVoiceEvents) -> ::windows_core::Result<()>;
    fn AlertBoundary(this: &Self::This) -> ::windows_core::Result<SpeechVoiceEvents>;
    fn SetSynchronousSpeakTimeout(this: &Self::This, mstimeout: i32) -> ::windows_core::Result<()>;
    fn SynchronousSpeakTimeout(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Speak(this: &Self::This, text: &::windows_core::BSTR, flags: SpeechVoiceSpeakFlags) -> ::windows_core::Result<i32>;
    fn SpeakStream(this: &Self::This, stream: ::core::option::Option<&ISpeechBaseStream>, flags: SpeechVoiceSpeakFlags) -> ::windows_core::Result<i32>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, r#type: &::windows_core::BSTR, numitems: i32) -> ::windows_core::Result<i32>;
    fn GetVoices(this: &Self::This, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
    fn GetAudioOutputs(this: &Self::This, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
    fn WaitUntilDone(this: &Self::This, mstimeout: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SpeakCompleteEvent(this: &Self::This) -> ::windows_core::Result<i32>;
    fn IsUISupported(this: &Self::This, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisplayUI(this: &Self::This, hwndparent: i32, title: &::windows_core::BSTR, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechVoice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechVoice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Voice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Voice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(voice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_Voice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_Voice(this, ::windows_core::from_raw_borrowed(&voice)).into())
        }
        unsafe extern "system" fn AudioOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiooutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioOutput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiooutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AudioOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiooutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AudioOutput(this, ::windows_core::from_raw_borrowed(&audiooutput)).into())
        }
        unsafe extern "system" fn AudioOutputStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiooutputstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioOutputStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiooutputstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AudioOutputStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiooutputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AudioOutputStream(this, ::windows_core::from_raw_borrowed(&audiooutputstream)).into())
        }
        unsafe extern "system" fn Rate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Rate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rate: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRate(this, ::core::mem::transmute_copy(&rate)).into())
        }
        unsafe extern "system" fn Volume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Volume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(volume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolume(this, ::core::mem::transmute_copy(&volume)).into())
        }
        unsafe extern "system" fn SetAllowAudioOutputFormatChangesOnNextSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowAudioOutputFormatChangesOnNextSet(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowAudioOutputFormatChangesOnNextSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowAudioOutputFormatChangesOnNextSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventInterests<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventinterestflags: *mut SpeechVoiceEvents) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventInterests(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventinterestflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventInterests<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventinterestflags: SpeechVoiceEvents) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventInterests(this, ::core::mem::transmute_copy(&eventinterestflags)).into())
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: SpeechVoicePriority) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&priority)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: *mut SpeechVoicePriority) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(priority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAlertBoundary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boundary: SpeechVoiceEvents) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlertBoundary(this, ::core::mem::transmute_copy(&boundary)).into())
        }
        unsafe extern "system" fn AlertBoundary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boundary: *mut SpeechVoiceEvents) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AlertBoundary(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(boundary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSynchronousSpeakTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mstimeout: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSynchronousSpeakTimeout(this, ::core::mem::transmute_copy(&mstimeout)).into())
        }
        unsafe extern "system" fn SynchronousSpeakTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mstimeout: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SynchronousSpeakTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mstimeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Speak<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Speak(this, ::core::mem::transmute(&text), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpeakStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpeakStream(this, ::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: ::std::mem::MaybeUninit<::windows_core::BSTR>, numitems: i32, numskipped: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Skip(this, ::core::mem::transmute(&r#type), ::core::mem::transmute_copy(&numitems)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numskipped, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVoices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVoices(this, ::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAudioOutputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAudioOutputs(this, ::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WaitUntilDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mstimeout: i32, done: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitUntilDone(this, ::core::mem::transmute_copy(&mstimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(done, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpeakCompleteEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpeakCompleteEvent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(handle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUISupported(this, ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::std::mem::MaybeUninit<::windows_core::BSTR>, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayUI(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&title), ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)).into())
        }
        ISpeechVoice_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            Voice: Voice::<Identity, Impl, OFFSET>,
            putref_Voice: putref_Voice::<Identity, Impl, OFFSET>,
            AudioOutput: AudioOutput::<Identity, Impl, OFFSET>,
            putref_AudioOutput: putref_AudioOutput::<Identity, Impl, OFFSET>,
            AudioOutputStream: AudioOutputStream::<Identity, Impl, OFFSET>,
            putref_AudioOutputStream: putref_AudioOutputStream::<Identity, Impl, OFFSET>,
            Rate: Rate::<Identity, Impl, OFFSET>,
            SetRate: SetRate::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            SetAllowAudioOutputFormatChangesOnNextSet: SetAllowAudioOutputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            AllowAudioOutputFormatChangesOnNextSet: AllowAudioOutputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            EventInterests: EventInterests::<Identity, Impl, OFFSET>,
            SetEventInterests: SetEventInterests::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Identity, Impl, OFFSET>,
            AlertBoundary: AlertBoundary::<Identity, Impl, OFFSET>,
            SetSynchronousSpeakTimeout: SetSynchronousSpeakTimeout::<Identity, Impl, OFFSET>,
            SynchronousSpeakTimeout: SynchronousSpeakTimeout::<Identity, Impl, OFFSET>,
            Speak: Speak::<Identity, Impl, OFFSET>,
            SpeakStream: SpeakStream::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            GetVoices: GetVoices::<Identity, Impl, OFFSET>,
            GetAudioOutputs: GetAudioOutputs::<Identity, Impl, OFFSET>,
            WaitUntilDone: WaitUntilDone::<Identity, Impl, OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechVoiceStatus_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CurrentStreamNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastStreamNumberQueued(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastHResult(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RunningState(this: &Self::This) -> ::windows_core::Result<SpeechRunState>;
    fn InputWordPosition(this: &Self::This) -> ::windows_core::Result<i32>;
    fn InputWordLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn InputSentencePosition(this: &Self::This) -> ::windows_core::Result<i32>;
    fn InputSentenceLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastBookmark(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LastBookmarkId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PhonemeId(this: &Self::This) -> ::windows_core::Result<i16>;
    fn VisemeId(this: &Self::This) -> ::windows_core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechVoiceStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechVoiceStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentStreamNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentStreamNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastStreamNumberQueued<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastStreamNumberQueued(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastHResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastHResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RunningState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRunState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RunningState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InputWordPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InputWordPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(position, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InputWordLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InputWordLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InputSentencePosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InputSentencePosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(position, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InputSentenceLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InputSentenceLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastBookmark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bookmark: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastBookmark(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bookmark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastBookmarkId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bookmarkid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastBookmarkId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bookmarkid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PhonemeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phoneid: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PhonemeId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phoneid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VisemeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visemeid: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VisemeId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visemeid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpeechVoiceStatus_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentStreamNumber: CurrentStreamNumber::<Identity, Impl, OFFSET>,
            LastStreamNumberQueued: LastStreamNumberQueued::<Identity, Impl, OFFSET>,
            LastHResult: LastHResult::<Identity, Impl, OFFSET>,
            RunningState: RunningState::<Identity, Impl, OFFSET>,
            InputWordPosition: InputWordPosition::<Identity, Impl, OFFSET>,
            InputWordLength: InputWordLength::<Identity, Impl, OFFSET>,
            InputSentencePosition: InputSentencePosition::<Identity, Impl, OFFSET>,
            InputSentenceLength: InputSentenceLength::<Identity, Impl, OFFSET>,
            LastBookmark: LastBookmark::<Identity, Impl, OFFSET>,
            LastBookmarkId: LastBookmarkId::<Identity, Impl, OFFSET>,
            PhonemeId: PhonemeId::<Identity, Impl, OFFSET>,
            VisemeId: VisemeId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechWaveFormatEx_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn FormatTag(this: &Self::This) -> ::windows_core::Result<i16>;
    fn SetFormatTag(this: &Self::This, formattag: i16) -> ::windows_core::Result<()>;
    fn Channels(this: &Self::This) -> ::windows_core::Result<i16>;
    fn SetChannels(this: &Self::This, channels: i16) -> ::windows_core::Result<()>;
    fn SamplesPerSec(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSamplesPerSec(this: &Self::This, samplespersec: i32) -> ::windows_core::Result<()>;
    fn AvgBytesPerSec(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAvgBytesPerSec(this: &Self::This, avgbytespersec: i32) -> ::windows_core::Result<()>;
    fn BlockAlign(this: &Self::This) -> ::windows_core::Result<i16>;
    fn SetBlockAlign(this: &Self::This, blockalign: i16) -> ::windows_core::Result<()>;
    fn BitsPerSample(this: &Self::This) -> ::windows_core::Result<i16>;
    fn SetBitsPerSample(this: &Self::This, bitspersample: i16) -> ::windows_core::Result<()>;
    fn ExtraData(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetExtraData(this: &Self::This, extradata: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechWaveFormatEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechWaveFormatEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FormatTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formattag: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatTag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(formattag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormatTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formattag: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormatTag(this, ::core::mem::transmute_copy(&formattag)).into())
        }
        unsafe extern "system" fn Channels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channels: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Channels(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(channels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetChannels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channels: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannels(this, ::core::mem::transmute_copy(&channels)).into())
        }
        unsafe extern "system" fn SamplesPerSec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, samplespersec: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SamplesPerSec(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(samplespersec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSamplesPerSec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, samplespersec: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSamplesPerSec(this, ::core::mem::transmute_copy(&samplespersec)).into())
        }
        unsafe extern "system" fn AvgBytesPerSec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, avgbytespersec: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AvgBytesPerSec(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(avgbytespersec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, avgbytespersec: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAvgBytesPerSec(this, ::core::mem::transmute_copy(&avgbytespersec)).into())
        }
        unsafe extern "system" fn BlockAlign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blockalign: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BlockAlign(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockalign, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBlockAlign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blockalign: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlockAlign(this, ::core::mem::transmute_copy(&blockalign)).into())
        }
        unsafe extern "system" fn BitsPerSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitspersample: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BitsPerSample(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitspersample, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBitsPerSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitspersample: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBitsPerSample(this, ::core::mem::transmute_copy(&bitspersample)).into())
        }
        unsafe extern "system" fn ExtraData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extradata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtraData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extradata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExtraData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extradata: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtraData(this, ::core::mem::transmute(&extradata)).into())
        }
        ISpeechWaveFormatEx_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FormatTag: FormatTag::<Identity, Impl, OFFSET>,
            SetFormatTag: SetFormatTag::<Identity, Impl, OFFSET>,
            Channels: Channels::<Identity, Impl, OFFSET>,
            SetChannels: SetChannels::<Identity, Impl, OFFSET>,
            SamplesPerSec: SamplesPerSec::<Identity, Impl, OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Identity, Impl, OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Identity, Impl, OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Identity, Impl, OFFSET>,
            BlockAlign: BlockAlign::<Identity, Impl, OFFSET>,
            SetBlockAlign: SetBlockAlign::<Identity, Impl, OFFSET>,
            BitsPerSample: BitsPerSample::<Identity, Impl, OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Identity, Impl, OFFSET>,
            ExtraData: ExtraData::<Identity, Impl, OFFSET>,
            SetExtraData: SetExtraData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechXMLRecoResult_Impl: ::windows_core::BaseImpl + ISpeechRecoResult_Impl {
    fn GetXMLResult(this: &Self::This, options: SPXMLRESULTOPTIONS) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetXMLErrorInfo(this: &Self::This, linenumber: *mut i32, scriptline: *mut ::windows_core::BSTR, source: *mut ::windows_core::BSTR, description: *mut ::windows_core::BSTR, resultcode: *mut i32, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpeechXMLRecoResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpeechRecoResult);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechXMLRecoResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechXMLRecoResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetXMLResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechXMLRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXMLResult(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechXMLRecoResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, source: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, resultcode: *mut i32, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetXMLErrorInfo(this, ::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)).into())
        }
        ISpeechXMLRecoResult_Vtbl {
            base__: <ISpeechRecoResult as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ISpeechRecoContextEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _ISpeechRecoContextEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _ISpeechRecoContextEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _ISpeechRecoContextEvents {
    const VTABLE: Self::Vtable = { _ISpeechRecoContextEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ISpeechVoiceEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _ISpeechVoiceEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _ISpeechVoiceEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _ISpeechVoiceEvents {
    const VTABLE: Self::Vtable = { _ISpeechVoiceEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
