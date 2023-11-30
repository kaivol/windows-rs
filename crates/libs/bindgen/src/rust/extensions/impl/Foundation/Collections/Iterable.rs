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
