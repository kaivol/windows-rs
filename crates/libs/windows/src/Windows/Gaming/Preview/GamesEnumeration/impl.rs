#[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`"]
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
pub trait IGameListEntry_Impl: ::windows_core::BaseImpl {
    fn DisplayInfo(this: &Self::This) -> ::windows_core::Result<super::super::super::ApplicationModel::AppDisplayInfo>;
    fn LaunchAsync(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn Category(this: &Self::This) -> ::windows_core::Result<GameListCategory>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>;
    fn SetCategoryAsync(this: &Self::This, value: GameListCategory) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
impl ::windows_core::Iids for IGameListEntry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameListEntry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGameListEntry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisplayInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameListEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LaunchAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameListEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LaunchAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Category<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameListEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GameListCategory) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Category(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameListEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCategoryAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameListEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: GameListCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetCategoryAsync(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGameListEntry_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisplayInfo: DisplayInfo::<Identity, Impl, OFFSET>,
            LaunchAsync: LaunchAsync::<Identity, Impl, OFFSET>,
            Category: Category::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            SetCategoryAsync: SetCategoryAsync::<Identity, Impl, OFFSET>,
        }
    };
}
