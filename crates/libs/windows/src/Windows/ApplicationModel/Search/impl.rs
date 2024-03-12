#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait ISearchPaneQueryChangedEventArgs_Impl: ::windows_core::BaseImpl {
    fn QueryText(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Language(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn LinguisticDetails(this: &Self::This) -> ::windows_core::Result<SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for ISearchPaneQueryChangedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchPaneQueryChangedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Language(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LinguisticDetails<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LinguisticDetails(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchPaneQueryChangedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryText: QueryText::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            LinguisticDetails: LinguisticDetails::<Identity, Impl, OFFSET>,
        }
    };
}
