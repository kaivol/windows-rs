#![allow(non_snake_case)]

use windows::core::*;
use windows::Foundation::Collections::*;
use windows::Foundation::*;
use windows::Win32::Foundation::E_BOUNDS;

#[implement(
    IIterable<T>,
    IVectorView<T>,
)]
struct Thing<T>(Vec<T::Default>)
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq;

impl<T> IVectorView_Impl<T> for Thing<T>
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq,
{
    fn GetAt(this: &Self::This, index: u32) -> Result<T> {
        match this.0.get(index as usize) {
            Some(value) => T::from_default(value),
            None => Err(Error::new(E_BOUNDS, "".into())),
        }
    }

    fn Size(this: &Self::This) -> Result<u32> {
        Ok(this.0.len() as u32)
    }

    fn IndexOf(this: &Self::This, value: &T::Default, result: &mut u32) -> Result<bool> {
        match this.0.iter().position(|element| element == value) {
            Some(index) => {
                *result = index as u32;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn GetMany(_this: &Self::This, _startindex: u32, _items: &mut [T::Default]) -> Result<u32> {
        panic!();
    }
}

impl<T> IIterable_Impl<T> for Thing<T>
where
    T: RuntimeType + 'static,
    <T as Type<T>>::Default: PartialEq,
{
    fn First(_this: &Self::This) -> Result<IIterator<T>> {
        unimplemented!()
    }
}

#[test]
fn test_implement() -> Result<()> {
    let v: IVectorView<i32> = Thing::<i32>(vec![10, 20, 30]).into_interface();
    assert_eq!(10, v.GetAt(0)?);
    assert_eq!(20, v.GetAt(1)?);
    assert_eq!(30, v.GetAt(2)?);
    assert_eq!(3, v.Size()?);

    let mut index = 0;
    assert_eq!(true, v.IndexOf(20, &mut index)?);
    assert_eq!(1, index);
    assert_eq!(true, v.IndexOf(30, &mut index)?);
    assert_eq!(2, index);
    assert_eq!(false, v.IndexOf(123, &mut index)?);

    let v: IVectorView<HSTRING> =
        Thing::<HSTRING>(vec!["10".into(), "20".into(), "30".into()]).into_interface();
    assert_eq!("10", v.GetAt(0)?);
    assert_eq!("20", v.GetAt(1)?);
    assert_eq!("30", v.GetAt(2)?);
    assert_eq!(3, v.Size()?);

    let mut index = 0;
    assert_eq!(true, v.IndexOf(&HSTRING::from("20"), &mut index)?);
    assert_eq!(1, index);
    assert_eq!(true, v.IndexOf(&HSTRING::from("30"), &mut index)?);
    assert_eq!(2, index);
    assert_eq!(false, v.IndexOf(&HSTRING::from("123"), &mut index)?);

    let url1: HSTRING = "http://one/".into();
    let url2: HSTRING = "http://two/".into();
    let url3: HSTRING = "http://three/".into();
    let v: IVectorView<IStringable> = Thing::<IStringable>(vec![
        Some(Uri::CreateUri(&url1)?.cast()?),
        Some(Uri::CreateUri(&url2)?.cast()?),
        Some(Uri::CreateUri(&url3)?.cast()?),
    ])
    .into_interface();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);
    assert_eq!(3, v.Size()?);

    Ok(())
}
