use windows::core::*;
use windows::Foundation::Collections::*;
use windows::Win32::Foundation::E_BOUNDS;

#[implement(
    windows::Foundation::Collections::IIterator<T>,
)]
struct Iterator<T>(std::cell::UnsafeCell<(ComObject<Iterable<T>>, usize)>)
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq;

#[allow(non_snake_case)]
impl<T> IIterator_Impl<T> for Iterator<T>
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq,
{
    fn Current(this: &Self::This) -> Result<T> {
        unsafe {
            let this = this.0.get();
            let owner = &(*this).0;

            if owner.0.len() > (*this).1 {
                Ok(owner.0[(*this).1].clone())
            } else {
                Err(Error::new(E_BOUNDS, "".into()))
            }
        }
    }

    fn HasCurrent(this: &Self::This) -> Result<bool> {
        unsafe {
            let this = this.0.get();
            let owner = &(*this).0;
            Ok(owner.0.len() > (*this).1)
        }
    }

    fn MoveNext(this: &Self::This) -> Result<bool> {
        unsafe {
            let this = this.0.get();
            let owner = &(*this).0;
            (*this).1 += 1;
            Ok(owner.0.len() > (*this).1)
        }
    }

    fn GetMany(_this: &Self::This, _items: &mut [T::Default]) -> Result<u32> {
        panic!(); // TODO: arrays still need some work.
    }
}

#[implement(
    windows::Foundation::Collections::IIterable<T>,
)]
struct Iterable<T>(Vec<T>)
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq;

#[allow(non_snake_case)]
impl<T> IIterable_Impl<T> for Iterable<T>
where
    T: RuntimeType + 'static + Clone,
    <T as Type<T>>::Default: PartialEq,
{
    fn First(this: &Self::This) -> Result<IIterator<T>> {
        Ok(Iterator::<T>((this.clone(), 0).into()).into_interface())
    }
}

#[test]
fn test_collect() -> Result<()> {
    let source: IIterable<i32> = Iterable(vec![10, 20, 30]).into_interface();

    // TODO: not sure why this won't compile.
    // let values: Vec<i32> = source.collect();

    let mut values = Vec::<i32>::new();

    for value in source {
        values.push(value);
    }

    assert_eq!(values, &[10, 20, 30]);

    Ok(())
}

#[test]
fn test_explicit() -> Result<()> {
    let iterable: IIterable<i32> = Iterable(vec![10, 20, 30]).into_interface();
    let it1 = iterable.First()?;

    assert_eq!(it1.Current()?, 10);
    assert_eq!(it1.HasCurrent()?, true);
    assert_eq!(it1.MoveNext()?, true);

    assert_eq!(it1.Current()?, 20);
    assert_eq!(it1.HasCurrent()?, true);
    assert_eq!(it1.MoveNext()?, true);

    assert_eq!(it1.Current()?, 30);
    assert_eq!(it1.HasCurrent()?, true);
    assert_eq!(it1.MoveNext()?, false);

    assert_eq!(it1.Current().is_err(), true);
    assert_eq!(it1.HasCurrent()?, false);
    assert_eq!(it1.MoveNext()?, false);

    // The following just validates that iterators are independent and stable.

    let it2 = iterable.First()?;

    assert_eq!(it2.Current()?, 10);
    assert_eq!(it2.HasCurrent()?, true);
    assert_eq!(it1.Current().is_err(), true);
    assert_eq!(it1.HasCurrent()?, false);

    assert_eq!(it2.Current()?, 10);
    assert_eq!(it2.HasCurrent()?, true);
    assert_eq!(it1.Current().is_err(), true);
    assert_eq!(it1.HasCurrent()?, false);

    Ok(())
}
