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
