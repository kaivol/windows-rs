pub trait IJsonValue_Impl: ::windows_core::BaseImpl {
    fn ValueType(this: &Self::This) -> ::windows_core::Result<JsonValueType>;
    fn Stringify(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetString(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetNumber(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetBoolean(this: &Self::This) -> ::windows_core::Result<bool>;
    fn GetArray(this: &Self::This) -> ::windows_core::Result<JsonArray>;
    fn GetObject(this: &Self::This) -> ::windows_core::Result<JsonObject>;
}
impl ::windows_core::Iids for IJsonValue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJsonValue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ValueType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut JsonValueType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValueType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Stringify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Stringify(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBoolean<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBoolean(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetArray(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IJsonValue_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ValueType: ValueType::<Identity, Impl, OFFSET>,
            Stringify: Stringify::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetNumber: GetNumber::<Identity, Impl, OFFSET>,
            GetBoolean: GetBoolean::<Identity, Impl, OFFSET>,
            GetArray: GetArray::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    };
}
