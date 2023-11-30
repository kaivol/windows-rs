pub trait IIterable_Impl<T>: ::windows_core::BaseImpl
where
    T: ::windows_core::RuntimeType + 'static,
{
    fn First(this: &Self::This) -> ::windows_core::Result<IIterator<T>>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IIterable<T> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIterable_Impl<T>, const OFFSET: usize, T: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IIterable<T> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn First<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIterable_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::First(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IIterable_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            First: First::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IIterator_Impl<T>: ::windows_core::BaseImpl
where
    T: ::windows_core::RuntimeType + 'static,
{
    fn Current(this: &Self::This) -> ::windows_core::Result<T>;
    fn HasCurrent(this: &Self::This) -> ::windows_core::Result<bool>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<bool>;
    fn GetMany(this: &Self::This, items: &mut [<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<u32>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IIterator<T> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: usize, T: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IIterator<T> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Current<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<T>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Current(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasCurrent<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMany<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIterator_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *mut ::windows_core::AbiType<T>, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMany(this, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IIterator_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Current: Current::<T, Identity, Impl, OFFSET>,
            HasCurrent: HasCurrent::<T, Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<T, Identity, Impl, OFFSET>,
            GetMany: GetMany::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IKeyValuePair_Impl<K, V>: ::windows_core::BaseImpl
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    fn Key(this: &Self::This) -> ::windows_core::Result<K>;
    fn Value(this: &Self::This) -> ::windows_core::Result<V>;
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IKeyValuePair<K, V> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKeyValuePair_Impl<K, V>, const OFFSET: usize, K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IKeyValuePair<K, V> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Key<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKeyValuePair_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<K>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Key(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKeyValuePair_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<V>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IKeyValuePair_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Key: Key::<K, V, Identity, Impl, OFFSET>,
            Value: Value::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMap_Impl<K, V>: ::windows_core::BaseImpl + IIterable_Impl<IKeyValuePair<K, V>>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    fn Lookup(this: &Self::This, key: &<K as ::windows_core::Type<K>>::Default) -> ::windows_core::Result<V>;
    fn Size(this: &Self::This) -> ::windows_core::Result<u32>;
    fn HasKey(this: &Self::This, key: &<K as ::windows_core::Type<K>>::Default) -> ::windows_core::Result<bool>;
    fn GetView(this: &Self::This) -> ::windows_core::Result<IMapView<K, V>>;
    fn Insert(this: &Self::This, key: &<K as ::windows_core::Type<K>>::Default, value: &<V as ::windows_core::Type<V>>::Default) -> ::windows_core::Result<bool>;
    fn Remove(this: &Self::This, key: &<K as ::windows_core::Type<K>>::Default) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IMap<K, V> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IIterable<IKeyValuePair<K, V>> as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: usize, K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IMap<K, V> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Lookup<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, result__: *mut ::windows_core::AbiType<V>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Lookup(this, ::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasKey<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasKey(this, ::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetView<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Insert<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, value: ::windows_core::AbiType<V>, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Insert(this, ::core::mem::transmute(&key), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&key)).into())
        }
        unsafe extern "system" fn Clear<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMap_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IMap_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Lookup: Lookup::<K, V, Identity, Impl, OFFSET>,
            Size: Size::<K, V, Identity, Impl, OFFSET>,
            HasKey: HasKey::<K, V, Identity, Impl, OFFSET>,
            GetView: GetView::<K, V, Identity, Impl, OFFSET>,
            Insert: Insert::<K, V, Identity, Impl, OFFSET>,
            Remove: Remove::<K, V, Identity, Impl, OFFSET>,
            Clear: Clear::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMapChangedEventArgs_Impl<K>: ::windows_core::BaseImpl
where
    K: ::windows_core::RuntimeType + 'static,
{
    fn CollectionChange(this: &Self::This) -> ::windows_core::Result<CollectionChange>;
    fn Key(this: &Self::This) -> ::windows_core::Result<K>;
}
impl<K: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IMapChangedEventArgs<K> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapChangedEventArgs_Impl<K>, const OFFSET: usize, K: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IMapChangedEventArgs<K> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CollectionChange<K: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapChangedEventArgs_Impl<K>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CollectionChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Key<K: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapChangedEventArgs_Impl<K>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<K>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Key(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMapChangedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CollectionChange: CollectionChange::<K, Identity, Impl, OFFSET>,
            Key: Key::<K, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMapView_Impl<K, V>: ::windows_core::BaseImpl + IIterable_Impl<IKeyValuePair<K, V>>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    fn Lookup(this: &Self::This, key: &<K as ::windows_core::Type<K>>::Default) -> ::windows_core::Result<V>;
    fn Size(this: &Self::This) -> ::windows_core::Result<u32>;
    fn HasKey(this: &Self::This, key: &<K as ::windows_core::Type<K>>::Default) -> ::windows_core::Result<bool>;
    fn Split(this: &Self::This, first: &mut ::core::option::Option<IMapView<K, V>>, second: &mut ::core::option::Option<IMapView<K, V>>) -> ::windows_core::Result<()>;
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IMapView<K, V> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IIterable<IKeyValuePair<K, V>> as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: usize, K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IMapView<K, V> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Lookup<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, result__: *mut ::windows_core::AbiType<V>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Lookup(this, ::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasKey<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::AbiType<K>, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasKey(this, ::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Split<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapView_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, first: *mut *mut ::core::ffi::c_void, second: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Split(this, ::core::mem::transmute_copy(&first), ::core::mem::transmute_copy(&second)).into())
        }
        IMapView_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Lookup: Lookup::<K, V, Identity, Impl, OFFSET>,
            Size: Size::<K, V, Identity, Impl, OFFSET>,
            HasKey: HasKey::<K, V, Identity, Impl, OFFSET>,
            Split: Split::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IObservableMap_Impl<K, V>: ::windows_core::BaseImpl + IIterable_Impl<IKeyValuePair<K, V>> + IMap_Impl<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
{
    fn MapChanged(this: &Self::This, vhnd: ::core::option::Option<&MapChangedEventHandler<K, V>>) -> ::windows_core::Result<super::EventRegistrationToken>;
    fn RemoveMapChanged(this: &Self::This, token: &super::EventRegistrationToken) -> ::windows_core::Result<()>;
}
impl<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IObservableMap<K, V> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IIterable<IKeyValuePair<K, V>> as ::windows_core::ComInterface>::IID, <IMap<K, V> as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObservableMap_Impl<K, V>, const OFFSET: usize, K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IObservableMap<K, V> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MapChanged<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObservableMap_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MapChanged(this, ::windows_core::from_raw_borrowed(&vhnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveMapChanged<K: ::windows_core::RuntimeType + 'static, V: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObservableMap_Impl<K, V>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveMapChanged(this, ::core::mem::transmute(&token)).into())
        }
        IObservableMap_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MapChanged: MapChanged::<K, V, Identity, Impl, OFFSET>,
            RemoveMapChanged: RemoveMapChanged::<K, V, Identity, Impl, OFFSET>,
            K: ::core::marker::PhantomData::<K>,
            V: ::core::marker::PhantomData::<V>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IObservableVector_Impl<T>: ::windows_core::BaseImpl + IIterable_Impl<T> + IVector_Impl<T>
where
    T: ::windows_core::RuntimeType + 'static,
{
    fn VectorChanged(this: &Self::This, vhnd: ::core::option::Option<&VectorChangedEventHandler<T>>) -> ::windows_core::Result<super::EventRegistrationToken>;
    fn RemoveVectorChanged(this: &Self::This, token: &super::EventRegistrationToken) -> ::windows_core::Result<()>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IObservableVector<T> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IIterable<T> as ::windows_core::ComInterface>::IID, <IVector<T> as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObservableVector_Impl<T>, const OFFSET: usize, T: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IObservableVector<T> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn VectorChanged<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObservableVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VectorChanged(this, ::windows_core::from_raw_borrowed(&vhnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveVectorChanged<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObservableVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveVectorChanged(this, ::core::mem::transmute(&token)).into())
        }
        IObservableVector_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            VectorChanged: VectorChanged::<T, Identity, Impl, OFFSET>,
            RemoveVectorChanged: RemoveVectorChanged::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPropertySet_Impl: ::windows_core::BaseImpl + IIterable_Impl<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>> + IMap_Impl<::windows_core::HSTRING, ::windows_core::IInspectable> + IObservableMap_Impl<::windows_core::HSTRING, ::windows_core::IInspectable> {}
impl ::windows_core::Iids for IPropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IIterable<IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>> as ::windows_core::ComInterface>::IID, <IMap<::windows_core::HSTRING, ::windows_core::IInspectable> as ::windows_core::ComInterface>::IID, <IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable> as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertySet {
    const VTABLE: Self::Vtable = { IPropertySet_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVector_Impl<T>: ::windows_core::BaseImpl + IIterable_Impl<T>
where
    T: ::windows_core::RuntimeType + 'static,
{
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<T>;
    fn Size(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetView(this: &Self::This) -> ::windows_core::Result<IVectorView<T>>;
    fn IndexOf(this: &Self::This, value: &<T as ::windows_core::Type<T>>::Default, index: &mut u32) -> ::windows_core::Result<bool>;
    fn SetAt(this: &Self::This, index: u32, value: &<T as ::windows_core::Type<T>>::Default) -> ::windows_core::Result<()>;
    fn InsertAt(this: &Self::This, index: u32, value: &<T as ::windows_core::Type<T>>::Default) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, value: &<T as ::windows_core::Type<T>>::Default) -> ::windows_core::Result<()>;
    fn RemoveAtEnd(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMany(this: &Self::This, startindex: u32, items: &mut [<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<u32>;
    fn ReplaceAll(this: &Self::This, items: &[<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<()>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IVector<T> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IIterable<T> as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize, T: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IVector<T> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAt<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows_core::AbiType<T>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetView<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IndexOf<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows_core::AbiType<T>, index: *mut u32, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IndexOf(this, ::core::mem::transmute(&value), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAt<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows_core::AbiType<T>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, index, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn InsertAt<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: ::windows_core::AbiType<T>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, index, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn RemoveAt<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, index).into())
        }
        unsafe extern "system" fn Append<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows_core::AbiType<T>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn RemoveAtEnd<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAtEnd(this).into())
        }
        unsafe extern "system" fn Clear<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn GetMany<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut ::windows_core::AbiType<T>, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMany(this, startindex, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReplaceAll<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVector_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, items_array_size: u32, items: *const ::windows_core::AbiType<T>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReplaceAll(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&items), items_array_size as usize)).into())
        }
        IVector_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAt: GetAt::<T, Identity, Impl, OFFSET>,
            Size: Size::<T, Identity, Impl, OFFSET>,
            GetView: GetView::<T, Identity, Impl, OFFSET>,
            IndexOf: IndexOf::<T, Identity, Impl, OFFSET>,
            SetAt: SetAt::<T, Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<T, Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<T, Identity, Impl, OFFSET>,
            Append: Append::<T, Identity, Impl, OFFSET>,
            RemoveAtEnd: RemoveAtEnd::<T, Identity, Impl, OFFSET>,
            Clear: Clear::<T, Identity, Impl, OFFSET>,
            GetMany: GetMany::<T, Identity, Impl, OFFSET>,
            ReplaceAll: ReplaceAll::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVectorChangedEventArgs_Impl: ::windows_core::BaseImpl {
    fn CollectionChange(this: &Self::This) -> ::windows_core::Result<CollectionChange>;
    fn Index(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IVectorChangedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVectorChangedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVectorChangedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CollectionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVectorChangedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CollectionChange) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CollectionChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Index<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVectorChangedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Index(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVectorChangedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CollectionChange: CollectionChange::<Identity, Impl, OFFSET>,
            Index: Index::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IVectorView_Impl<T>: ::windows_core::BaseImpl + IIterable_Impl<T>
where
    T: ::windows_core::RuntimeType + 'static,
{
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<T>;
    fn Size(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IndexOf(this: &Self::This, value: &<T as ::windows_core::Type<T>>::Default, index: &mut u32) -> ::windows_core::Result<bool>;
    fn GetMany(this: &Self::This, startindex: u32, items: &mut [<T as ::windows_core::Type<T>>::Default]) -> ::windows_core::Result<u32>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IVectorView<T> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IIterable<T> as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: usize, T: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IVectorView<T> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAt<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows_core::AbiType<T>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IndexOf<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows_core::AbiType<T>, index: *mut u32, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IndexOf(this, ::core::mem::transmute(&value), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMany<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVectorView_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut ::windows_core::AbiType<T>, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMany(this, startindex, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&items), items_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVectorView_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAt: GetAt::<T, Identity, Impl, OFFSET>,
            Size: Size::<T, Identity, Impl, OFFSET>,
            IndexOf: IndexOf::<T, Identity, Impl, OFFSET>,
            GetMany: GetMany::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[::windows_implement::implement(IIterable<T>)]
struct StockIterable<T>
where
    T: ::windows_core::RuntimeType + 'static,
    <T as ::windows_core::Type<T>>::Default: ::std::clone::Clone,
{
    values: std::vec::Vec<T::Default>,
}

impl<T> IIterable_Impl<T> for StockIterable<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: ::std::clone::Clone,
{
    fn First(this: &Self::This) -> ::windows_core::Result<IIterator<T>> {
        use windows_core::ComObjectImplExt;
        Ok(StockIterator { owner: this.clone(), current: 0.into() }.into_interface())
    }
}

#[::windows_implement::implement(IIterator<T>)]
struct StockIterator<T>
where
    T: ::windows_core::RuntimeType + 'static,
    <T as ::windows_core::Type<T>>::Default: ::std::clone::Clone,
{
    owner: ::windows_core::ComObject<StockIterable<T>>,
    current: ::std::sync::atomic::AtomicUsize,
}

impl<T> IIterator_Impl<T> for StockIterator<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: ::std::clone::Clone,
{
    fn Current(this: &Self::This) -> ::windows_core::Result<T> {
        let current = this.current.load(::std::sync::atomic::Ordering::Relaxed);

        if this.owner.values.len() > current {
            T::from_default(&this.owner.values[current])
        } else {
            Err(::windows_core::Error::from(::windows_core::imp::E_BOUNDS))
        }
    }

    fn HasCurrent(this: &Self::This) -> ::windows_core::Result<bool> {
        let current = this.current.load(::std::sync::atomic::Ordering::Relaxed);

        Ok(this.owner.values.len() > current)
    }

    fn MoveNext(this: &Self::This) -> ::windows_core::Result<bool> {
        let current = this.current.load(::std::sync::atomic::Ordering::Relaxed);

        if current < this.owner.values.len() {
            this.current.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        }

        Ok(this.owner.values.len() > current + 1)
    }

    fn GetMany(this: &Self::This, values: &mut [T::Default]) -> ::windows_core::Result<u32> {
        let current = this.current.load(::std::sync::atomic::Ordering::Relaxed);

        let actual = std::cmp::min(this.owner.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&this.owner.values[current..current + actual]);
        this.current.fetch_add(actual, ::std::sync::atomic::Ordering::Relaxed);
        Ok(actual as _)
    }
}

impl<T> ::core::convert::TryFrom<::std::vec::Vec<T::Default>> for IIterable<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: ::std::clone::Clone,
{
    type Error = ::windows_core::Error;
    fn try_from(values: ::std::vec::Vec<T::Default>) -> ::windows_core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        use windows_core::ComObjectImplExt;
        Ok(StockIterable::<T> { values }.into_interface())
    }
}
#[::windows_implement::implement(IMapView<K, V>, IIterable<IKeyValuePair<K, V>>)]
struct StockMapView<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    map: std::collections::BTreeMap<K::Default, V::Default>,
}

impl<K, V> IIterable_Impl<IKeyValuePair<K, V>> for StockMapView<K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    fn First(this: &Self::This) -> ::windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        use windows_core::ComObjectImplExt;
        Ok(StockMapViewIterator::<K, V> { _owner: this.clone(), current: std::sync::RwLock::new(this.map.iter()) }.into_interface())
    }
}

impl<K, V> IMapView_Impl<K, V> for StockMapView<K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    fn Lookup(this: &Self::This, key: &K::Default) -> ::windows_core::Result<V> {
        let value = this.map.get(key).ok_or_else(|| ::windows_core::Error::from(::windows_core::imp::E_BOUNDS))?;
        V::from_default(value)
    }
    fn Size(this: &Self::This) -> ::windows_core::Result<u32> {
        Ok(this.map.len().try_into()?)
    }
    fn HasKey(this: &Self::This, key: &K::Default) -> ::windows_core::Result<bool> {
        Ok(this.map.contains_key(key))
    }
    fn Split(_: &Self::This, first: &mut std::option::Option<IMapView<K, V>>, second: &mut std::option::Option<IMapView<K, V>>) -> ::windows_core::Result<()> {
        *first = None;
        *second = None;
        Ok(())
    }
}

#[::windows_implement::implement(IIterator<IKeyValuePair<K, V>>)]
struct StockMapViewIterator<'a, K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    _owner: ::windows_core::ComObject<StockMapView<K, V>>,
    current: ::std::sync::RwLock<std::collections::btree_map::Iter<'a, K::Default, V::Default>>,
}

impl<'a, K, V> IIterator_Impl<IKeyValuePair<K, V>> for StockMapViewIterator<'a, K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    fn Current(this: &Self::This) -> ::windows_core::Result<IKeyValuePair<K, V>> {
        let mut current = this.current.read().unwrap().clone().peekable();

        if let Some((key, value)) = current.peek() {
            use windows_core::ComObjectImplExt;
            Ok(StockKeyValuePair::<K, V> { key: (*key).clone(), value: (*value).clone() }.into_interface())
        } else {
            Err(::windows_core::Error::from(::windows_core::imp::E_BOUNDS))
        }
    }

    fn HasCurrent(this: &Self::This) -> ::windows_core::Result<bool> {
        let mut current = this.current.read().unwrap().clone().peekable();

        Ok(current.peek().is_some())
    }

    fn MoveNext(this: &Self::This) -> ::windows_core::Result<bool> {
        let mut current = this.current.write().unwrap();

        current.next();
        Ok(current.clone().peekable().peek().is_some())
    }

    fn GetMany(this: &Self::This, pairs: &mut [Option<IKeyValuePair<K, V>>]) -> ::windows_core::Result<u32> {
        let mut current = this.current.write().unwrap();
        let mut actual = 0;

        for pair in pairs {
            if let Some((key, value)) = current.next() {
                use windows_core::ComObjectImplExt;
                *pair = Some(StockKeyValuePair::<K, V> { key: (*key).clone(), value: (*value).clone() }.into_interface());
                actual += 1;
            } else {
                break;
            }
        }

        Ok(actual as _)
    }
}

#[::windows_implement::implement(IKeyValuePair<K, V>)]
struct StockKeyValuePair<K, V>
where
    K: ::windows_core::RuntimeType + 'static,
    V: ::windows_core::RuntimeType + 'static,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    key: K::Default,
    value: V::Default,
}

impl<K, V> IKeyValuePair_Impl<K, V> for StockKeyValuePair<K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    fn Key(this: &Self::This) -> ::windows_core::Result<K> {
        K::from_default(&this.key)
    }
    fn Value(this: &Self::This) -> ::windows_core::Result<V> {
        V::from_default(&this.value)
    }
}

impl<K, V> ::core::convert::TryFrom<std::collections::BTreeMap<K::Default, V::Default>> for IMapView<K, V>
where
    K: ::windows_core::RuntimeType,
    V: ::windows_core::RuntimeType,
    <K as ::windows_core::Type<K>>::Default: std::clone::Clone + std::cmp::Ord,
    <V as ::windows_core::Type<V>>::Default: std::clone::Clone,
{
    type Error = ::windows_core::Error;
    fn try_from(map: std::collections::BTreeMap<K::Default, V::Default>) -> ::windows_core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        use windows_core::ComObjectImplExt;
        Ok(StockMapView::<K, V> { map }.into_interface())
    }
}
#[::windows_implement::implement(IVectorView<T>, IIterable<T>)]
struct StockVectorView<T>
where
    T: ::windows_core::RuntimeType + 'static,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    values: std::vec::Vec<T::Default>,
}

impl<T> IIterable_Impl<T> for StockVectorView<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    fn First(this: &Self::This) -> ::windows_core::Result<IIterator<T>> {
        use windows_core::ComObjectImplExt;
        Ok(StockVectorViewIterator { owner: this.clone(), current: 0.into() }.into_interface())
    }
}

impl<T> IVectorView_Impl<T> for StockVectorView<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<T> {
        let item = this.values.get(index as usize).ok_or_else(|| ::windows_core::Error::from(::windows_core::imp::E_BOUNDS))?;
        T::from_default(item)
    }
    fn Size(this: &Self::This) -> ::windows_core::Result<u32> {
        Ok(this.values.len().try_into()?)
    }
    fn IndexOf(this: &Self::This, value: &T::Default, result: &mut u32) -> ::windows_core::Result<bool> {
        match this.values.iter().position(|element| element == value) {
            Some(index) => {
                *result = index as _;
                Ok(true)
            }
            None => Ok(false),
        }
    }
    fn GetMany(this: &Self::This, current: u32, values: &mut [T::Default]) -> ::windows_core::Result<u32> {
        let current = current as usize;
        if current >= this.values.len() {
            return Ok(0);
        }
        let actual = std::cmp::min(this.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&this.values[current..current + actual]);
        Ok(actual as _)
    }
}

#[::windows_implement::implement(IIterator<T>)]
struct StockVectorViewIterator<T>
where
    T: ::windows_core::RuntimeType + 'static,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    owner: ::windows_core::ComObject<StockVectorView<T>>,
    current: ::std::sync::atomic::AtomicUsize,
}

impl<T> IIterator_Impl<T> for StockVectorViewIterator<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    fn Current(this: &Self::This) -> ::windows_core::Result<T> {
        let current = this.current.load(::std::sync::atomic::Ordering::Relaxed);

        if this.owner.values.len() > current {
            T::from_default(&this.owner.values[current])
        } else {
            Err(::windows_core::Error::from(::windows_core::imp::E_BOUNDS))
        }
    }

    fn HasCurrent(this: &Self::This) -> ::windows_core::Result<bool> {
        let current = this.current.load(::std::sync::atomic::Ordering::Relaxed);

        Ok(this.owner.values.len() > current)
    }

    fn MoveNext(this: &Self::This) -> ::windows_core::Result<bool> {
        let current = this.current.load(::std::sync::atomic::Ordering::Relaxed);

        if current < this.owner.values.len() {
            this.current.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        }

        Ok(this.owner.values.len() > current + 1)
    }

    fn GetMany(this: &Self::This, values: &mut [T::Default]) -> ::windows_core::Result<u32> {
        let current = this.current.load(::std::sync::atomic::Ordering::Relaxed);

        let actual = std::cmp::min(this.owner.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&this.owner.values[current..current + actual]);
        this.current.fetch_add(actual, ::std::sync::atomic::Ordering::Relaxed);
        Ok(actual as _)
    }
}

impl<T> ::core::convert::TryFrom<::std::vec::Vec<T::Default>> for IVectorView<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    type Error = ::windows_core::Error;
    fn try_from(values: ::std::vec::Vec<T::Default>) -> ::windows_core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        use windows_core::ComObjectImplExt;
        Ok(StockVectorView::<T> { values }.into_interface())
    }
}
