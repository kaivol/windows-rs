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
        Ok(StockMapViewIterator::<K, V> {
            _owner: this.clone(),
            current: std::sync::RwLock::new(this.map.iter())
        }.into_interface())
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
                *pair = Some(StockKeyValuePair::<K,V> { key: (*key).clone(), value: (*value).clone() }.into_interface());
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
